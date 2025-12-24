module.exports = async ({ github, context, process }) => {
    const marker = '<!-- ci:unit-tests -->';

    const buildOutcome = process.env.BUILD_OUTCOME;
    const testOutcome = process.env.TEST_OUTCOME;

    let body;

    if (testOutcome === 'success') {
        if (buildOutcome === 'success') {
            body = `${marker}\n🎉 **All unit tests passed!** ✅`;
        } else if (buildOutcome === 'failure') {
            body = `${marker}\n⚠️ **Unit tests passed, but the build step failed. Please check the CI logs.**`;
        } else {
            body = `${marker}\n⚠️ **All unit tests passed, but the build step had outcome: \`${buildOutcome}\`. Please check the CI logs.**`;
        }
    } else if (testOutcome === 'failure') {
        if (buildOutcome === 'failure') {
            body = `${marker}\n💥 **Build and unit tests failed. Please check the CI logs.** ❌`;
        } else if (buildOutcome === 'cancelled') {
            body = `${marker}\n⚠️ **Build was cancelled, but unit tests reported failures.** ⏹️❌`;
        } else if (buildOutcome === 'skipped') {
            body = `${marker}\n⚠️ **Build was skipped, but unit tests reported failures. Please check the CI logs.** ⏭️❌`;
        } else {
            body = `${marker}\n💥 **Unit tests failed. Please check the CI logs.** ❌`;
        }
    } else if (testOutcome === 'cancelled') {
        body = `${marker}\n⚠️ **Unit tests were cancelled.** ⏹️`;
    } else if (testOutcome === 'skipped') {
        if (buildOutcome === 'failure') {
            body = `${marker}\n⚠️ **Unit tests were skipped because the build step failed.** 🏗️❌`;
        } else {
            body = `${marker}\n⚠️ **Unit tests were skipped.**`;
        }
    } else {
        body = `${marker}\n⚠️ **Unknown test status. Please check the CI logs.**`;
    }

    await github.rest.issues.createComment({
        owner: context.repo.owner,
        repo: context.repo.repo,
        issue_number: context.issue.number,
        body
    });
};
