#!/usr/bin/env bash
set -euo pipefail

: "${GH_TOKEN:?GH_TOKEN is required}"
: "${REPO:?REPO is required}"
: "${PR_NUMBER:?PR_NUMBER is required}"

marker='<!-- ci:unit-tests -->'

build_outcome="${BUILD_OUTCOME:-}"
test_outcome="${TEST_OUTCOME:-}"

case "$test_outcome" in
	success)
		case "$build_outcome" in
			success) message='🎉 **All unit tests passed!** ✅' ;;
			failure) message='⚠️ **Unit tests passed, but the build step failed. Please check the CI logs.**' ;;
			*) message="⚠️ **All unit tests passed, but the build step had outcome: \`${build_outcome}\`. Please check the CI logs.**" ;;
		esac
		;;
	failure)
		case "$build_outcome" in
			failure) message='💥 **Build and unit tests failed. Please check the CI logs.** ❌' ;;
			cancelled) message='⚠️ **Build was cancelled, but unit tests reported failures.** ⏹️❌' ;;
			skipped) message='⚠️ **Build was skipped, but unit tests reported failures. Please check the CI logs.** ⏭️❌' ;;
			*) message='💥 **Unit tests failed. Please check the CI logs.** ❌' ;;
		esac
		;;
	cancelled)
		message='⚠️ **Unit tests were cancelled.** ⏹️'
		;;
	skipped)
		if [ "$build_outcome" = 'failure' ]; then
			message='⚠️ **Unit tests were skipped because the build step failed.** 🏗️❌'
		else
			message='⚠️ **Unit tests were skipped.**'
		fi
		;;
	*)
		message='⚠️ **Unknown test status. Please check the CI logs.**'
		;;
esac

body="$(printf '%s\n%s' "$marker" "$message")"

create_comment() {
	gh api "repos/${REPO}/issues/${PR_NUMBER}/comments" -f body="$body" > /dev/null
}

# Prevent comment spam: update the existing marker comment if present
if ! comment_id="$(gh api "repos/${REPO}/issues/${PR_NUMBER}/comments" --paginate \
	--jq ".[] | select(.user.login == \"github-actions[bot]\" and (.body // \"\" | contains(\"${marker}\"))) | .id" \
	| head -n 1)"; then
	# fallback: just make a comment, but also fail to surface the error
	create_comment
	exit 1
fi

if [ -n "$comment_id" ]; then
	gh api -X PATCH "repos/${REPO}/issues/comments/${comment_id}" -f body="$body" > /dev/null
else
	create_comment
fi
