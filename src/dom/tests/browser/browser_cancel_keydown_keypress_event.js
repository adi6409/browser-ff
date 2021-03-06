const URL =
  "https://example.com/browser/dom/tests/browser/prevent_return_key.html";

// Wait for alert dialog and dismiss it immediately.
function awaitAndCloseAlertDialog() {
  return new Promise(resolve => {
    function onDialogShown(node) {
      Services.obs.removeObserver(onDialogShown, "tabmodal-dialog-loaded");
      let button = node.querySelector(".tabmodalprompt-button0");
      button.click();
      resolve();
    }
    Services.obs.addObserver(onDialogShown, "tabmodal-dialog-loaded");
  });
}

add_task(async function() {
  await SpecialPowers.pushPrefEnv({
    set: [["prompts.contentPromptSubDialog", false]],
  });

  let tab = await BrowserTestUtils.openNewForegroundTab(gBrowser, URL);
  let browser = tab.linkedBrowser;

  // Focus and enter random text on input.
  await SpecialPowers.spawn(browser, [], async function() {
    let input = content.document.getElementById("input");
    input.focus();
    input.value = "abcd";
  });

  // Send return key (cross process) to submit the form implicitly.
  let dialogShown = awaitAndCloseAlertDialog();
  EventUtils.synthesizeKey("KEY_Enter");
  await dialogShown;

  // Check that the form should not have been submitted.
  await SpecialPowers.spawn(browser, [], async function() {
    let result = content.document.getElementById("result").innerHTML;
    info("submit result: " + result);
    is(result, "not submitted", "form should not have submitted");
  });

  gBrowser.removeCurrentTab();
});
