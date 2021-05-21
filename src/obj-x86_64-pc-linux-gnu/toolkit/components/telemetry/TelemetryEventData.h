/* This file is auto-generated, only for internal use in TelemetryEvent.h,
   see gen_event_data.py. */

#ifndef mozilla_TelemetryEventData_h
#define mozilla_TelemetryEventData_h
#include "core/EventInfo.h"
namespace {

#if defined(_MSC_VER) && !defined(__clang__)
const uint32_t gExtraKeysTable[] = {
#else
constexpr uint32_t gExtraKeysTable[] = {
#endif
  // activity_stream, [end], [session]
  // extra_keys: addon_version, page, session_id, user_prefs
  0, 14, 19, 30,
  // activity_stream, [event], [ARCHIVE_FROM_POCKET, BLOCK, BOOKMARK_ADD, BOOKMARK_DELETE, CLICK, CLICK_PRIVACY_INFO, CLOSE_NEWTAB_PREFS, SHOW_PERSONALIZE, HIDE_PERSONALIZE, DELETE, DELETE_FROM_POCKET, DELETE_CONFIRM, DIALOG_CANCEL, DIALOG_OPEN, DRAG, DROP, MIGRATION_CANCEL, MIGRATION_START, OPEN_NEWTAB_PREFS, OPEN_NEW_WINDOW, OPEN_PRIVATE_WINDOW, PIN, PREF_CHANGED, PREVIEW_REQUEST, SAVE_TO_POCKET, SEARCH, SEARCH_EDIT_ADD, SEARCH_EDIT_CLOSE, SEARCH_EDIT_DELETE, SEARCH_HANDOFF, SHOW_PRIVACY_INFO, SKIPPED_SIGNIN, SUBMIT_EMAIL, DISCLAIMER_ACKED, MENU_ADD_SEARCH, MENU_ADD_TOPSITE, MENU_COLLAPSE, MENU_EXPAND, MENU_MANAGE, MENU_MOVE_DOWN, MENU_MOVE_UP, MENU_PRIVACY_NOTICE, MENU_REMOVE, TOP_SITES_EDIT, TOP_SITES_EDIT_CLOSE, TOPSITE_SPONSOR_INFO, UNPIN]
  // extra_keys: action_position, addon_version, page, session_id, user_prefs
  41, 0, 14, 19, 30,
  // addonsManager, [action], [aboutAddons, browserAction, customize, pageAction, doorhanger, appUpgrade]
  // extra_keys: action, addonId, type, view
  57, 64, 72, 77,
  // addonsManager, [install, update], [extension, theme, locale, dictionary, other, unknown]
  // extra_keys: addon_id, download_time, error, method, num_strings, source, step, updated_from
  82, 91, 105, 111, 118, 130, 137, 142,
  // addonsManager, [install_stats], [extension, theme, locale, dictionary, other, unknown]
  // extra_keys: addon_id, utm_campaign, utm_content, utm_medium, utm_source
  82, 155, 168, 180, 191,
  // addonsManager, [link], [aboutAddons, aboutPreferences, customize]
  // extra_keys: type, view
  72, 77,
  // addonsManager, [disable, enable, sideload_prompt, uninstall], [extension, theme, locale, dictionary, other]
  // extra_keys: method, num_strings, source
  111, 118, 130,
  // addonsManager, [report], [amo, menu, toolbar_context_menu, uninstall]
  // extra_keys: addon_type, error_type
  202, 213,
  // addonsManager, [view], [aboutAddons]
  // extra_keys: addonId, source, type
  64, 130, 72,
  // creditcard, [detected, popup_shown, filled, filled_modified, submitted], [cc_form]
  // extra_keys: cc_exp, cc_exp_found, cc_name, cc_name_found, cc_number, cc_number_found, field_name, fields_auto, fields_modified, fields_not_auto
  224, 231, 244, 252, 266, 276, 292, 303, 315, 331,
  // devtools.main, [activate], [responsive_design, split_console]
  // extra_keys: host, session_id, width
  347, 19, 352,
  // devtools.main, [add_breakpoint], [debugger]
  // extra_keys: session_id
  19,
  // devtools.main, [blackbox], [debugger]
  // extra_keys: session_id
  19,
  // devtools.main, [close], [tools]
  // extra_keys: host, session_id, width
  347, 19, 352,
  // devtools.main, [close_adbg], [aboutdebugging]
  // extra_keys: session_id, width
  19, 352,
  // devtools.main, [close_source_tab], [debugger]
  // extra_keys: num_tabs, reason
  358, 367,
  // devtools.main, [connection_attempt], [aboutdebugging]
  // extra_keys: connection_id, connection_type, runtime_id, session_id, status
  374, 388, 404, 19, 415,
  // devtools.main, [continue], [debugger]
  // extra_keys: session_id
  19,
  // devtools.main, [deactivate], [responsive_design, split_console]
  // extra_keys: host, session_id, width
  347, 19, 352,
  // devtools.main, [device_added], [aboutdebugging]
  // extra_keys: connection_type, device_name, session_id
  388, 422, 19,
  // devtools.main, [device_removed], [aboutdebugging]
  // extra_keys: connection_type, device_name, session_id
  388, 422, 19,
  // devtools.main, [edit_html], [inspector]
  // extra_keys: made_changes, session_id, time_open
  434, 19, 447,
  // devtools.main, [edit_resend], [netmonitor]
  // extra_keys: session_id
  19,
  // devtools.main, [edit_rule], [ruleview]
  // extra_keys: session_id
  19,
  // devtools.main, [enter], [accessibility, application, dom, inspector, jsdebugger, memory, netmonitor, options, performance, storage, styleeditor, webconsole, whatsnew, other, fakeTool4242, testBlankPanel, testTool, testtool1, testTool1072208, testtool2]
  // extra_keys: cold, host, message_count, panel_name, session_id, start_state, width
  457, 347, 462, 476, 19, 487, 352,
  // devtools.main, [execute_js], [webconsole]
  // extra_keys: input, lines, session_id
  499, 505, 19,
  // devtools.main, [exit], [accessibility, application, dom, inspector, jsdebugger, memory, netmonitor, options, performance, storage, styleeditor, webconsole, whatsnew, other, fakeTool4242, testBlankPanel, testTool, testtool1, testTool1072208, testtool2]
  // extra_keys: host, next_panel, panel_name, reason, session_id, width
  347, 511, 476, 367, 19, 352,
  // devtools.main, [f12_enabled], [tools]
  // extra_keys: session_id
  19,
  // devtools.main, [f12_popup_displayed], [tools]
  // extra_keys: session_id
  19,
  // devtools.main, [filters_changed], [netmonitor, webconsole]
  // extra_keys: active, inactive, session_id, trigger
  522, 529, 19, 538,
  // devtools.main, [inspect], [aboutdebugging]
  // extra_keys: runtime_type, session_id, target_type
  546, 19, 559,
  // devtools.main, [jump_to_definition], [webconsole]
  // extra_keys: session_id
  19,
  // devtools.main, [jump_to_source], [webconsole]
  // extra_keys: session_id
  19,
  // devtools.main, [object_expanded], [webconsole]
  // extra_keys: session_id
  19,
  // devtools.main, [open], [tools]
  // extra_keys: entrypoint, first_panel, host, session_id, shortcut, splitconsole, width
  571, 582, 347, 19, 594, 603, 352,
  // devtools.main, [open_adbg], [aboutdebugging]
  // extra_keys: session_id, width
  19, 352,
  // devtools.main, [pause], [debugger]
  // extra_keys: lib_stacks, reason, session_id
  616, 367, 19,
  // devtools.main, [pause_on_exceptions], [debugger]
  // extra_keys: caught_exceptio, exceptions, session_id
  627, 643, 19,
  // devtools.main, [persist_changed], [netmonitor, webconsole]
  // extra_keys: session_id
  19,
  // devtools.main, [pretty_print], [debugger]
  // extra_keys: session_id
  19,
  // devtools.main, [remove_breakpoint], [debugger]
  // extra_keys: session_id
  19,
  // devtools.main, [reverse_search], [webconsole]
  // extra_keys: functionality, session_id
  654, 19,
  // devtools.main, [runtime_added], [aboutdebugging]
  // extra_keys: connection_type, device_name, runtime_id, runtime_name, session_id
  388, 422, 404, 668, 19,
  // devtools.main, [runtime_connected], [aboutdebugging]
  // extra_keys: connection_type, device_name, runtime_id, runtime_name, runtime_os, runtime_version, session_id
  388, 422, 404, 668, 681, 692, 19,
  // devtools.main, [runtime_disconnected], [aboutdebugging]
  // extra_keys: connection_type, device_name, runtime_id, runtime_name, session_id
  388, 422, 404, 668, 19,
  // devtools.main, [runtime_removed], [aboutdebugging]
  // extra_keys: connection_type, device_name, runtime_id, runtime_name, session_id
  388, 422, 404, 668, 19,
  // devtools.main, [select_page], [aboutdebugging, application]
  // extra_keys: page_type, session_id
  708, 19,
  // devtools.main, [select_ws_frame], [netmonitor]
  // extra_keys: session_id
  19,
  // devtools.main, [show_profiler], [aboutdebugging]
  // extra_keys: runtime_id, session_id
  404, 19,
  // devtools.main, [sidepanel_changed], [inspector, netmonitor]
  // extra_keys: newpanel, oldpanel, os, session_id
  718, 727, 736, 19,
  // devtools.main, [start_worker], [application]
  // extra_keys: session_id
  19,
  // devtools.main, [throttle_changed], [netmonitor]
  // extra_keys: mode, session_id
  739, 19,
  // devtools.main, [tool_timer], [animationinspector, compatibilityview, computedview, changesview, fontinspector, layoutview, ruleview]
  // extra_keys: os, session_id, time_open
  736, 19, 447,
  // devtools.main, [unregister_worker], [application]
  // extra_keys: session_id
  19,
  // devtools.main, [update_conn_prompt], [aboutdebugging]
  // extra_keys: prompt_enabled, runtime_id, session_id
  744, 404, 19,
  // doh, [evaluate], [heuristics]
  // extra_keys: browserParent, canary, evaluateReason, google, modifiedRoots, policy, steeredProvider, thirdPartyRoots, youtube, zscalerCanary
  759, 773, 780, 795, 802, 816, 823, 839, 855, 863,
  // doh, [evaluate_v2], [heuristics]
  // extra_keys: canaries, captiveState, enterprise, evaluateReason, filtering, networkID, platform, steeredProvider
  877, 886, 899, 780, 910, 920, 930, 823,
  // dom.quota.try, [error], [step]
  // extra_keys: context, module, result, seq, source_file, source_line
  939, 947, 954, 961, 965, 977,
  // extensions.data, [migrateResult], [storageLocal]
  // extra_keys: backend, data_migrated, error_name, has_jsonfile, has_olddata
  989, 997, 1011, 1022, 1035,
  // extensions.data, [storageLocalError], [get, set, remove, clear]
  // extra_keys: error_name
  1011,
  // form_autocomplete, [show], [logins]
  // extra_keys: acFieldName, fieldType, generatedPasswo, hadPrevious, importableLogin, insecureWarning, login, loginsFooter, stringLength, typeWasPassword
  1047, 1059, 1069, 1085, 1097, 1113, 1129, 1135, 1148, 1161,
  // fxa, [connect, disconnect], [account]
  // extra_keys: fxa, sync
  1177, 1181,
  // fxa_app_menu, [click], [account_settings, cad, login, send_tab, sync_now, sync_settings, sync_tabs, sync_tabs_sidebar, toolbar_icon, unver_sync_settings, open_monitor, open_send]
  // extra_keys: fxa_avatar, fxa_status
  1186, 1197,
  // fxa_avatar_menu, [click], [account_settings, cad, login, send_tab, sync_now, sync_settings, sync_tabs, sync_tabs_sidebar, toolbar_icon, unver_sync_settings, open_monitor, open_send]
  // extra_keys: fxa_avatar, fxa_status
  1186, 1197,
  // homepage, [preference], [ignore]
  // extra_keys: webExtensionId
  1208,
  // installation, [first_seen], [full, stub]
  // extra_keys: admin_user, build_id, default_path, from_msi, install_existed, profdir_existed, silent, version
  1223, 1234, 1243, 1256, 1265, 1281, 1297, 1304,
  // intl.ui.browserLanguage, [manage, search, add, remove, reorder, apply, accept, cancel], [dialog, main]
  // extra_keys: installId
  1312,
  // messaging_experiments, [reach], [cfr, whats_new_panel, moments_page, snippets, cfr_fxa]
  // extra_keys: branches
  1322,
  // navigation, [search], [about_home, about_newtab, contextmenu, oneoff, suggestion, alias, enter, searchbar, urlbar, urlbar_searchmode, webextension]
  // extra_keys: engine
  1331,
  // network.dns, [trrConfirmation], [context]
  // extra_keys: attemptCount, captivePortal, contextReason, failedLookups, networkID, results, time, trigger
  1338, 1351, 1365, 1379, 920, 1393, 1401, 538,
  // normandy, [enroll], [preference_study, addon_study, preference_rollout, addon_rollout]
  // extra_keys: addonId, addonVersion, branch, enrollmentId, experimentType
  64, 1406, 1419, 1426, 1439,
  // normandy, [enrollFailed], [addon_study, preference_rollout, preference_study, addon_rollout]
  // extra_keys: addonId, branch, conflictingSlug, detail, enrollmentId, preference, reason
  64, 1419, 1454, 1470, 1426, 1477, 367,
  // normandy, [expose], [feature_study]
  // extra_keys: branchSlug, featureId
  1488, 1499,
  // normandy, [graduate], [preference_rollout]
  // extra_keys: enrollmentId, reason
  1426, 367,
  // normandy, [unenroll], [preference_study, addon_study, preference_rollback, addon_rollback]
  // extra_keys: addonId, addonVersion, branch, changedPref, didResetValue, enrollmentId, reason
  64, 1406, 1419, 1509, 1521, 1426, 367,
  // normandy, [unenrollFailed], [preference_rollback, preference_study, addon_rollback]
  // extra_keys: caller, changedPref, enrollmentId, originalReason, reason
  1535, 1509, 1426, 1542, 367,
  // normandy, [update], [addon_study, preference_rollout, addon_rollout]
  // extra_keys: addonId, addonVersion, branch, enrollmentId, previousState
  64, 1406, 1419, 1426, 1557,
  // normandy, [updateFailed], [addon_study, addon_rollout]
  // extra_keys: branch, detail, enrollmentId, reason
  1419, 1470, 1426, 367,
  // pictureinpicture, [create], [player]
  // extra_keys: height, screenX, screenY, width
  1571, 1578, 1586, 352,
  // pictureinpicture, [move], [player]
  // extra_keys: screenX, screenY
  1578, 1586,
  // pictureinpicture, [resize], [player]
  // extra_keys: height, width
  1571, 352,
  // pwmgr, [doorhanger_submitted], [save, update]
  // extra_keys: did_edit_pw, did_edit_un, did_select_pw, did_select_un
  1594, 1606, 1618, 1632,
  // pwmgr, [cancel, copy, delete, dismiss_breach_alert, edit, filter, hide, learn_more_breach, learn_more_vuln, new, open_site, save, select, show, sort], [existing_login, list, new_login, password, username]
  // extra_keys: breached, sort_key, vulnerable
  1646, 1655, 1664,
  // pwmgr, [reauthenticate], [master_password, os_auth]
  // extra_keys: auto_admin, require_signon
  1675, 1686,
  // pwmgr, [saved_login_used], [form_login, form_password, auth_login, prompt_login]
  // extra_keys: filled
  1701,
  // readermode, [view], [on, off]
  // extra_keys: subcategory
  1708,
  // security, [evalUsage], [systemContext, parentProcess]
  // extra_keys: fileinfo
  1720,
  // security, [fissionPrincipals], [contentParent]
  // extra_keys: principalType, scheme
  1729, 1743,
  // security, [javascriptLoad], [parentProcess]
  // extra_keys: fileinfo
  1720,
  // security, [unexpectedload], [systemprincipal]
  // extra_keys: contenttype, filedetails, remotetype
  1750, 1762, 1774,
  // security.doh.trrPerformance, [resolved], [record]
  // extra_keys: captivePortal, domain, networkUnstable, retryCount, status, time, trr
  1351, 1785, 1792, 1808, 415, 1401, 1819,
  // security.ui.certerror, [click], [advanced_button, exception_button, return_button_top, return_button_adv, learn_more_link, auto_report_cb, error_code_link, clipboard_button_top, clipboard_button_bot]
  // extra_keys: has_sts, is_frame, panel_open
  1823, 1831, 1840,
  // security.ui.certerror, [load], [aboutcerterror]
  // extra_keys: has_sts, is_frame
  1823, 1831,
  // security.ui.protections, [click], [lw_open_button, lw_sync_link, lw_about_link, mtr_about_link, mtr_report_link, mtr_signup_button, trackers_about_link, mobile_app_link, settings_link, vpn_banner_link, vpn_banner_close, vpn_card_link, vpn_app_link_android, vpn_app_link_ios]
  // extra_keys: category
  1851,
  // security.ui.protections, [close], [protection_report]
  // extra_keys: category
  1851,
  // security.ui.protections, [show], [protection_report, vpn_banner]
  // extra_keys: category
  1851,
  // slow_script_warning, [shown], [browser, content]
  // extra_keys: end_reason, hang_duration, n_tab_deselect, uptime, uri_type, wait_count
  1860, 1871, 1885, 1900, 1907, 1916,
  // telemetry.test, [content_only], [object1]
  // extra_keys: bar, foo
  1927, 1931,
  // telemetry.test, [optout], [object1, object2]
  // extra_keys: key1
  1935,
  // telemetry.test, [test1, test2], [object1, object2]
  // extra_keys: key1, key2
  1935, 1940,
  // telemetry.test.second, [test], [object1, object2, object3]
  // extra_keys: key1
  1935,
  // uptake.remotecontent.result, [uptake], [remotesettings, normandy]
  // extra_keys: age, duration, errorName, source, timestamp, trigger
  1945, 1949, 1958, 130, 1968, 538,
  // urlbar, [abandonment], [blur]
  // extra_keys: elapsed, numChars, numWords
  1978, 1986, 1995,
  // urlbar, [engagement], [click, enter, paste_go, drop_go]
  // extra_keys: elapsed, numChars, numWords, provider, selIndex, selType
  1978, 1986, 1995, 2004, 2013, 2022,
  // webrtc.ui, [share_display], [screen, window, browser_window]
  // extra_keys: silence_notifs
  2030,
  // ysod, [shown], [ysod]
  // extra_keys: error_code, last_line, last_line_len, location
  2045, 2056, 2066, 2080,
  // zero_byte_load, [load], [ftl, dtd, properties, js, xml, xhtml, css, json, html, png, svg, others]
  // extra_keys: cancelled, file_name, status, sync
  2089, 2099, 415, 1181,
};
static_assert(sizeof(gExtraKeysTable) <= UINT32_MAX, "index overflow");

#if defined(_MSC_VER) && !defined(__clang__)
const CommonEventInfo gCommonEventInfo[] = {
#else
constexpr CommonEventInfo gCommonEventInfo[] = {
#endif
  // category: activity_stream
  // methods: [end]
  // objects: [session]
  {2109, 2125, 0, 4, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: activity_stream
  // methods: [event]
  // objects: [ARCHIVE_FROM_POCKET, BLOCK, BOOKMARK_ADD, BOOKMARK_DELETE, CLICK, CLICK_PRIVACY_INFO, CLOSE_NEWTAB_PREFS, SHOW_PERSONALIZE, HIDE_PERSONALIZE, DELETE, DELETE_FROM_POCKET, DELETE_CONFIRM, DIALOG_CANCEL, DIALOG_OPEN, DRAG, DROP, MIGRATION_CANCEL, MIGRATION_START, OPEN_NEWTAB_PREFS, OPEN_NEW_WINDOW, OPEN_PRIVATE_WINDOW, PIN, PREF_CHANGED, PREVIEW_REQUEST, SAVE_TO_POCKET, SEARCH, SEARCH_EDIT_ADD, SEARCH_EDIT_CLOSE, SEARCH_EDIT_DELETE, SEARCH_HANDOFF, SHOW_PRIVACY_INFO, SKIPPED_SIGNIN, SUBMIT_EMAIL, DISCLAIMER_ACKED, MENU_ADD_SEARCH, MENU_ADD_TOPSITE, MENU_COLLAPSE, MENU_EXPAND, MENU_MANAGE, MENU_MOVE_DOWN, MENU_MOVE_UP, MENU_PRIVACY_NOTICE, MENU_REMOVE, TOP_SITES_EDIT, TOP_SITES_EDIT_CLOSE, TOPSITE_SPONSOR_INFO, UNPIN]
  {2109, 2125, 4, 5, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: addonsManager
  // methods: [action]
  // objects: [aboutAddons, browserAction, customize, pageAction, doorhanger, appUpgrade]
  {2131, 2145, 9, 4, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: addonsManager
  // methods: [install, update]
  // objects: [extension, theme, locale, dictionary, other, unknown]
  {2131, 2145, 13, 8, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec | mozilla::Telemetry::Common::SupportedProduct::Thunderbird },
  // category: addonsManager
  // methods: [install_stats]
  // objects: [extension, theme, locale, dictionary, other, unknown]
  {2131, 2125, 21, 5, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: addonsManager
  // methods: [link]
  // objects: [aboutAddons, aboutPreferences, customize]
  {2131, 2145, 26, 2, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: addonsManager
  // methods: [disable, enable, sideload_prompt, uninstall]
  // objects: [extension, theme, locale, dictionary, other]
  {2131, 2145, 28, 3, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec | mozilla::Telemetry::Common::SupportedProduct::Thunderbird },
  // category: addonsManager
  // methods: [report]
  // objects: [amo, menu, toolbar_context_menu, uninstall]
  {2131, 2145, 31, 2, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec | mozilla::Telemetry::Common::SupportedProduct::Thunderbird },
  // category: addonsManager
  // methods: [view]
  // objects: [aboutAddons]
  {2131, 2145, 33, 3, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: avif
  // methods: [dav1d_get_picture]
  // objects: [return_value]
  {2152, 2125, 0, 0, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::All, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: contextservices.quicksuggest
  // methods: [enable_toggled]
  // objects: [enabled, disabled]
  {2157, 2125, 0, 0, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: creditcard
  // methods: [detected, popup_shown, filled, filled_modified, submitted]
  // objects: [cc_form]
  {2186, 2197, 36, 10, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Content, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: creditcard
  // methods: [show, save, update, cancel, disable]
  // objects: [capture_doorhanger, update_doorhanger]
  {2186, 2197, 0, 0, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: creditcard
  // methods: [show, add, delete, show_entry, edit]
  // objects: [manage]
  {2186, 2197, 0, 0, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: devtools.main
  // methods: [activate]
  // objects: [responsive_design, split_console]
  {2204, 2125, 46, 3, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [add_breakpoint]
  // objects: [debugger]
  {2204, 2125, 49, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [blackbox]
  // objects: [debugger]
  {2204, 2125, 50, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [close]
  // objects: [tools]
  {2204, 2125, 51, 3, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [close_adbg]
  // objects: [aboutdebugging]
  {2204, 2125, 54, 2, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [close_source_tab]
  // objects: [debugger]
  {2204, 2218, 56, 2, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: devtools.main
  // methods: [connection_attempt]
  // objects: [aboutdebugging]
  {2204, 2125, 58, 5, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [continue]
  // objects: [debugger]
  {2204, 2125, 63, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [deactivate]
  // objects: [responsive_design, split_console]
  {2204, 2125, 64, 3, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [device_added]
  // objects: [aboutdebugging]
  {2204, 2125, 67, 3, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [device_removed]
  // objects: [aboutdebugging]
  {2204, 2125, 70, 3, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [edit_html]
  // objects: [inspector]
  {2204, 2125, 73, 3, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [edit_resend]
  // objects: [netmonitor]
  {2204, 2125, 76, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [edit_rule]
  // objects: [ruleview]
  {2204, 2125, 77, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [enter]
  // objects: [accessibility, application, dom, inspector, jsdebugger, memory, netmonitor, options, performance, storage, styleeditor, webconsole, whatsnew, other, fakeTool4242, testBlankPanel, testTool, testtool1, testTool1072208, testtool2]
  {2204, 2125, 78, 7, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [execute_js]
  // objects: [webconsole]
  {2204, 2125, 85, 3, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [exit]
  // objects: [accessibility, application, dom, inspector, jsdebugger, memory, netmonitor, options, performance, storage, styleeditor, webconsole, whatsnew, other, fakeTool4242, testBlankPanel, testTool, testtool1, testTool1072208, testtool2]
  {2204, 2125, 88, 6, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [f12_enabled]
  // objects: [tools]
  {2204, 2225, 94, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: devtools.main
  // methods: [f12_popup_displayed]
  // objects: [tools]
  {2204, 2225, 95, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: devtools.main
  // methods: [filters_changed]
  // objects: [netmonitor, webconsole]
  {2204, 2125, 96, 4, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [inspect]
  // objects: [aboutdebugging]
  {2204, 2125, 100, 3, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [jump_to_definition]
  // objects: [webconsole]
  {2204, 2125, 103, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [jump_to_source]
  // objects: [webconsole]
  {2204, 2125, 104, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [object_expanded]
  // objects: [webconsole]
  {2204, 2125, 105, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [open]
  // objects: [tools]
  {2204, 2125, 106, 7, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [open_adbg]
  // objects: [aboutdebugging]
  {2204, 2125, 113, 2, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [pause]
  // objects: [debugger]
  {2204, 2125, 115, 3, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [pause_on_exceptions]
  // objects: [debugger]
  {2204, 2125, 118, 3, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [persist_changed]
  // objects: [netmonitor, webconsole]
  {2204, 2125, 121, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [pretty_print]
  // objects: [debugger]
  {2204, 2125, 122, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [remove_breakpoint]
  // objects: [debugger]
  {2204, 2125, 123, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [reverse_search]
  // objects: [webconsole]
  {2204, 2125, 124, 2, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [runtime_added]
  // objects: [aboutdebugging]
  {2204, 2125, 126, 5, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [runtime_connected]
  // objects: [aboutdebugging]
  {2204, 2125, 131, 7, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [runtime_disconnected]
  // objects: [aboutdebugging]
  {2204, 2125, 138, 5, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [runtime_removed]
  // objects: [aboutdebugging]
  {2204, 2125, 143, 5, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [select_page]
  // objects: [aboutdebugging, application]
  {2204, 2125, 148, 2, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [select_ws_frame]
  // objects: [netmonitor]
  {2204, 2125, 150, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [show_profiler]
  // objects: [aboutdebugging]
  {2204, 2125, 151, 2, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [sidepanel_changed]
  // objects: [inspector, netmonitor]
  {2204, 2125, 153, 4, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [start_worker]
  // objects: [application]
  {2204, 2125, 157, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: devtools.main
  // methods: [throttle_changed]
  // objects: [netmonitor]
  {2204, 2125, 158, 2, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [tool_timer]
  // objects: [animationinspector, compatibilityview, computedview, changesview, fontinspector, layoutview, ruleview]
  {2204, 2125, 160, 3, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: devtools.main
  // methods: [unregister_worker]
  // objects: [application]
  {2204, 2125, 163, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: devtools.main
  // methods: [update_conn_prompt]
  // objects: [aboutdebugging]
  {2204, 2125, 164, 3, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: doh
  // methods: [evaluate]
  // objects: [heuristics]
  {2232, 2125, 167, 10, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: doh
  // methods: [evaluate_v2]
  // objects: [heuristics]
  {2232, 2125, 177, 8, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: doh
  // methods: [state]
  // objects: [enabled, disabled, manuallyDisabled, policyDisabled, uninstalled, UIOk, UIDisabled, rollback, shutdown]
  {2232, 2125, 0, 0, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: dom.quota.try
  // methods: [error]
  // objects: [step]
  {2236, 2125, 185, 6, nsITelemetry::DATASET_PRERELEASE_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main | mozilla::Telemetry::Common::RecordedProcessType::Content, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: downloads
  // methods: [added]
  // objects: [fileExtension]
  {2250, 2125, 0, 0, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: extensions.data
  // methods: [migrateResult]
  // objects: [storageLocal]
  {2260, 2145, 191, 5, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: extensions.data
  // methods: [storageLocalError]
  // objects: [get, set, remove, clear]
  {2260, 2276, 196, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main | mozilla::Telemetry::Common::RecordedProcessType::Content, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: form_autocomplete
  // methods: [show]
  // objects: [logins]
  {2283, 2125, 197, 10, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: fxa
  // methods: [connect, disconnect]
  // objects: [account]
  {1177, 2125, 207, 2, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: fxa_app_menu
  // methods: [click]
  // objects: [account_settings, cad, login, send_tab, sync_now, sync_settings, sync_tabs, sync_tabs_sidebar, toolbar_icon, unver_sync_settings, open_monitor, open_send]
  {2301, 2125, 209, 2, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: fxa_avatar_menu
  // methods: [click]
  // objects: [account_settings, cad, login, send_tab, sync_now, sync_settings, sync_tabs, sync_tabs_sidebar, toolbar_icon, unver_sync_settings, open_monitor, open_send]
  {2314, 2125, 211, 2, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: homepage
  // methods: [preference]
  // objects: [ignore]
  {2330, 2125, 213, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: installation
  // methods: [first_seen]
  // objects: [full, stub]
  {2339, 2352, 214, 8, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: intl.ui.browserLanguage
  // methods: [manage, search, add, remove, reorder, apply, accept, cancel]
  // objects: [dialog, main]
  {2359, 2383, 222, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: jsonfile
  // methods: [load]
  // objects: [logins, autofillprofiles]
  {2390, 2125, 0, 0, nsITelemetry::DATASET_PRERELEASE_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: messaging_experiments
  // methods: [reach]
  // objects: [cfr, whats_new_panel, moments_page, snippets, cfr_fxa]
  {2399, 2125, 223, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: messaging_experiments
  // methods: [targeting]
  // objects: [attribute_error, attribute_timeout]
  {2399, 2125, 0, 0, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: navigation
  // methods: [search]
  // objects: [about_home, about_newtab, contextmenu, oneoff, suggestion, alias, enter, searchbar, urlbar, urlbar_searchmode, webextension]
  {2421, 2125, 224, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: network.dns
  // methods: [trrConfirmation]
  // objects: [context]
  {2432, 2125, 225, 8, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: normandy
  // methods: [enroll]
  // objects: [preference_study, addon_study, preference_rollout, addon_rollout]
  {2444, 2125, 233, 5, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: normandy
  // methods: [enrollFailed]
  // objects: [addon_study, preference_rollout, preference_study, addon_rollout]
  {2444, 2125, 238, 7, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: normandy
  // methods: [expose]
  // objects: [feature_study]
  {2444, 2125, 245, 2, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main | mozilla::Telemetry::Common::RecordedProcessType::Content, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: normandy
  // methods: [graduate]
  // objects: [preference_rollout]
  {2444, 2125, 247, 2, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: normandy
  // methods: [unenroll]
  // objects: [preference_study, addon_study, preference_rollback, addon_rollback]
  {2444, 2125, 249, 7, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: normandy
  // methods: [unenrollFailed]
  // objects: [preference_rollback, preference_study, addon_rollback]
  {2444, 2125, 256, 5, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: normandy
  // methods: [update]
  // objects: [addon_study, preference_rollout, addon_rollout]
  {2444, 2125, 261, 5, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: normandy
  // methods: [updateFailed]
  // objects: [addon_study, addon_rollout]
  {2444, 2125, 266, 4, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: partner_link
  // methods: [attribution]
  // objects: [success, failure, abort]
  {2453, 2125, 0, 0, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: partner_link
  // methods: [click]
  // objects: [newtab, urlbar]
  {2453, 2125, 0, 0, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: pictureinpicture
  // methods: [create]
  // objects: [player]
  {2466, 2145, 270, 4, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: pictureinpicture
  // methods: [move]
  // objects: [player]
  {2466, 2145, 274, 2, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: pictureinpicture
  // methods: [resize]
  // objects: [player]
  {2466, 2145, 276, 2, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: pwmgr
  // methods: [autocomplete_field, autocomplete_shown]
  // objects: [generatedpassword]
  {2483, 2125, 0, 0, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: pwmgr
  // methods: [doorhanger_submitted]
  // objects: [save, update]
  {2483, 2145, 278, 4, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: pwmgr
  // methods: [filled_field_edited]
  // objects: [generatedpassword]
  {2483, 2125, 0, 0, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Thunderbird },
  // category: pwmgr
  // methods: [cancel, copy, delete, dismiss_breach_alert, edit, filter, hide, learn_more_breach, learn_more_vuln, new, open_site, save, select, show, sort]
  // objects: [existing_login, list, new_login, password, username]
  {2483, 2125, 282, 3, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Content, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: pwmgr
  // methods: [mgmt_menu_item_used]
  // objects: [import_from_browser, import_from_csv, import_csv_complete, export, export_complete, preferences]
  {2483, 2125, 0, 0, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Content | mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: pwmgr
  // methods: [open_management]
  // objects: [aboutprotections, autocomplete, capturedoorhanger, contextmenu, direct, fxamenu, mainmenu, pageinfo, preferences, snippet]
  {2483, 2125, 0, 0, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main | mozilla::Telemetry::Common::RecordedProcessType::Content, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: pwmgr
  // methods: [reauthenticate]
  // objects: [master_password, os_auth]
  {2483, 2125, 285, 2, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main | mozilla::Telemetry::Common::RecordedProcessType::Content, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: pwmgr
  // methods: [saved_login_used]
  // objects: [form_login, form_password, auth_login, prompt_login]
  {2483, 2125, 287, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: readermode
  // methods: [view]
  // objects: [on, off]
  {2489, 2125, 288, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Content, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: security
  // methods: [evalUsage]
  // objects: [systemContext, parentProcess]
  {2500, 2125, 289, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::All, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: security
  // methods: [fissionPrincipals]
  // objects: [contentParent]
  {2500, 2125, 290, 2, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: security
  // methods: [javascriptLoad]
  // objects: [parentProcess]
  {2500, 2125, 292, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: security
  // methods: [unexpectedload]
  // objects: [systemprincipal]
  {2500, 2125, 293, 3, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::All, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: security.doh.trrPerformance
  // methods: [resolved]
  // objects: [record]
  {2509, 2125, 296, 7, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: security.doh.trrPerformance
  // methods: [trrselect]
  // objects: [dryrunresult]
  {2509, 2125, 0, 0, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: security.ui.app_menu
  // methods: [click]
  // objects: [open_full_report]
  {2537, 2383, 0, 0, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: security.ui.certerror
  // methods: [click]
  // objects: [advanced_button, exception_button, return_button_top, return_button_adv, learn_more_link, auto_report_cb, error_code_link, clipboard_button_top, clipboard_button_bot]
  {2558, 2125, 303, 3, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Content, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: security.ui.certerror
  // methods: [load]
  // objects: [aboutcerterror]
  {2558, 2125, 306, 2, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Content, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: security.ui.protections
  // methods: [click]
  // objects: [lw_open_button, lw_sync_link, lw_about_link, mtr_about_link, mtr_report_link, mtr_signup_button, trackers_about_link, mobile_app_link, settings_link, vpn_banner_link, vpn_banner_close, vpn_card_link, vpn_app_link_android, vpn_app_link_ios]
  {2580, 2383, 308, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Content, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: security.ui.protections
  // methods: [close]
  // objects: [protection_report]
  {2580, 2383, 309, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Content, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: security.ui.protections
  // methods: [show]
  // objects: [protection_report, vpn_banner]
  {2580, 2383, 310, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Content, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: security.ui.protectionspopup
  // methods: [click]
  // objects: [etp_toggle_on, etp_toggle_off, sitenotworking_link, send_report_link, send_report_submit, social, cookies, trackers, fingerprinters, cryptominers, subview_settings, settings, full_report, milestone_message]
  {2604, 2383, 0, 0, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: security.ui.protectionspopup
  // methods: [open]
  // objects: [protections_popup]
  {2604, 2383, 0, 0, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: slow_script_warning
  // methods: [shown]
  // objects: [browser, content]
  {2633, 2197, 311, 6, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: telemetry.test
  // methods: [content_only]
  // objects: [object1]
  {2653, 2125, 317, 2, nsITelemetry::DATASET_PRERELEASE_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Content, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec | mozilla::Telemetry::Common::SupportedProduct::Thunderbird },
  // category: telemetry.test
  // methods: [default_products]
  // objects: [object1]
  {2653, 2125, 0, 0, nsITelemetry::DATASET_PRERELEASE_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec | mozilla::Telemetry::Common::SupportedProduct::Thunderbird },
  // category: telemetry.test
  // methods: [desktop_only]
  // objects: [object1]
  {2653, 2125, 0, 0, nsITelemetry::DATASET_PRERELEASE_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Thunderbird },
  // category: telemetry.test
  // methods: [expired_version]
  // objects: [object1, object2]
  {2653, 2668, 0, 0, nsITelemetry::DATASET_PRERELEASE_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec | mozilla::Telemetry::Common::SupportedProduct::Thunderbird },
  // category: telemetry.test
  // methods: [main_and_content]
  // objects: [object1]
  {2653, 2125, 0, 0, nsITelemetry::DATASET_PRERELEASE_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main | mozilla::Telemetry::Common::RecordedProcessType::Content, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec | mozilla::Telemetry::Common::SupportedProduct::Thunderbird },
  // category: telemetry.test
  // methods: [main_only]
  // objects: [object1]
  {2653, 2125, 0, 0, nsITelemetry::DATASET_PRERELEASE_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec | mozilla::Telemetry::Common::SupportedProduct::Thunderbird },
  // category: telemetry.test
  // methods: [mobile_only]
  // objects: [object1]
  {2653, 2125, 0, 0, nsITelemetry::DATASET_PRERELEASE_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Fennec },
  // category: telemetry.test
  // methods: [multiproduct]
  // objects: [object1]
  {2653, 2125, 0, 0, nsITelemetry::DATASET_PRERELEASE_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec | mozilla::Telemetry::Common::SupportedProduct::Thunderbird },
  // category: telemetry.test
  // methods: [not_expired_optout]
  // objects: [object1]
  {2653, 2674, 0, 0, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec | mozilla::Telemetry::Common::SupportedProduct::Thunderbird },
  // category: telemetry.test
  // methods: [optout]
  // objects: [object1, object2]
  {2653, 2125, 319, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec | mozilla::Telemetry::Common::SupportedProduct::Thunderbird },
  // category: telemetry.test
  // methods: [test1, test2]
  // objects: [object1, object2]
  {2653, 2125, 320, 2, nsITelemetry::DATASET_PRERELEASE_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec | mozilla::Telemetry::Common::SupportedProduct::Thunderbird },
  // category: telemetry.test.second
  // methods: [test]
  // objects: [object1, object2, object3]
  {2682, 2125, 322, 1, nsITelemetry::DATASET_PRERELEASE_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec | mozilla::Telemetry::Common::SupportedProduct::Thunderbird },
  // category: uptake.remotecontent.result
  // methods: [uptake]
  // objects: [remotesettings, normandy]
  {2704, 2125, 323, 6, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox | mozilla::Telemetry::Common::SupportedProduct::Fennec | mozilla::Telemetry::Common::SupportedProduct::Thunderbird },
  // category: urlbar
  // methods: [abandonment]
  // objects: [blur]
  {2732, 2125, 329, 3, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: urlbar
  // methods: [engagement]
  // objects: [click, enter, paste_go, drop_go]
  {2732, 2125, 332, 6, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: webrtc.ui
  // methods: [share_display]
  // objects: [screen, window, browser_window]
  {2739, 2749, 338, 1, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: webrtc.ui
  // methods: [show_indicator]
  // objects: [show_indicator]
  {2739, 2756, 0, 0, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: ysod
  // methods: [shown]
  // objects: [ysod]
  {2763, 2125, 339, 4, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main, mozilla::Telemetry::Common::SupportedProduct::Firefox },
  // category: zero_byte_load
  // methods: [load]
  // objects: [ftl, dtd, properties, js, xml, xhtml, css, json, html, png, svg, others]
  {2768, 2125, 343, 4, nsITelemetry::DATASET_ALL_CHANNELS, mozilla::Telemetry::Common::RecordedProcessType::Main | mozilla::Telemetry::Common::RecordedProcessType::Content, mozilla::Telemetry::Common::SupportedProduct::Firefox },
};
static_assert(sizeof(gCommonEventInfo) <= UINT32_MAX, "index overflow");

#if defined(_MSC_VER) && !defined(__clang__)
const EventInfo gEventInfo[] = {
#else
constexpr EventInfo gEventInfo[] = {
#endif
  // category: activity_stream, method: end, object: session
  {gCommonEventInfo[0], 2783, 2787},
  // category: activity_stream, method: event, object: ARCHIVE_FROM_POCKET
  {gCommonEventInfo[1], 2795, 2801},
  // category: activity_stream, method: event, object: BLOCK
  {gCommonEventInfo[1], 2795, 2821},
  // category: activity_stream, method: event, object: BOOKMARK_ADD
  {gCommonEventInfo[1], 2795, 2827},
  // category: activity_stream, method: event, object: BOOKMARK_DELETE
  {gCommonEventInfo[1], 2795, 2840},
  // category: activity_stream, method: event, object: CLICK
  {gCommonEventInfo[1], 2795, 2856},
  // category: activity_stream, method: event, object: CLICK_PRIVACY_INFO
  {gCommonEventInfo[1], 2795, 2862},
  // category: activity_stream, method: event, object: CLOSE_NEWTAB_PREFS
  {gCommonEventInfo[1], 2795, 2881},
  // category: activity_stream, method: event, object: SHOW_PERSONALIZE
  {gCommonEventInfo[1], 2795, 2900},
  // category: activity_stream, method: event, object: HIDE_PERSONALIZE
  {gCommonEventInfo[1], 2795, 2917},
  // category: activity_stream, method: event, object: DELETE
  {gCommonEventInfo[1], 2795, 2934},
  // category: activity_stream, method: event, object: DELETE_FROM_POCKET
  {gCommonEventInfo[1], 2795, 2941},
  // category: activity_stream, method: event, object: DELETE_CONFIRM
  {gCommonEventInfo[1], 2795, 2960},
  // category: activity_stream, method: event, object: DIALOG_CANCEL
  {gCommonEventInfo[1], 2795, 2975},
  // category: activity_stream, method: event, object: DIALOG_OPEN
  {gCommonEventInfo[1], 2795, 2989},
  // category: activity_stream, method: event, object: DRAG
  {gCommonEventInfo[1], 2795, 3001},
  // category: activity_stream, method: event, object: DROP
  {gCommonEventInfo[1], 2795, 3006},
  // category: activity_stream, method: event, object: MIGRATION_CANCEL
  {gCommonEventInfo[1], 2795, 3011},
  // category: activity_stream, method: event, object: MIGRATION_START
  {gCommonEventInfo[1], 2795, 3028},
  // category: activity_stream, method: event, object: OPEN_NEWTAB_PREFS
  {gCommonEventInfo[1], 2795, 3044},
  // category: activity_stream, method: event, object: OPEN_NEW_WINDOW
  {gCommonEventInfo[1], 2795, 3062},
  // category: activity_stream, method: event, object: OPEN_PRIVATE_WINDOW
  {gCommonEventInfo[1], 2795, 3078},
  // category: activity_stream, method: event, object: PIN
  {gCommonEventInfo[1], 2795, 3098},
  // category: activity_stream, method: event, object: PREF_CHANGED
  {gCommonEventInfo[1], 2795, 3102},
  // category: activity_stream, method: event, object: PREVIEW_REQUEST
  {gCommonEventInfo[1], 2795, 3115},
  // category: activity_stream, method: event, object: SAVE_TO_POCKET
  {gCommonEventInfo[1], 2795, 3131},
  // category: activity_stream, method: event, object: SEARCH
  {gCommonEventInfo[1], 2795, 3146},
  // category: activity_stream, method: event, object: SEARCH_EDIT_ADD
  {gCommonEventInfo[1], 2795, 3153},
  // category: activity_stream, method: event, object: SEARCH_EDIT_CLOSE
  {gCommonEventInfo[1], 2795, 3169},
  // category: activity_stream, method: event, object: SEARCH_EDIT_DELETE
  {gCommonEventInfo[1], 2795, 3187},
  // category: activity_stream, method: event, object: SEARCH_HANDOFF
  {gCommonEventInfo[1], 2795, 3206},
  // category: activity_stream, method: event, object: SHOW_PRIVACY_INFO
  {gCommonEventInfo[1], 2795, 3221},
  // category: activity_stream, method: event, object: SKIPPED_SIGNIN
  {gCommonEventInfo[1], 2795, 3239},
  // category: activity_stream, method: event, object: SUBMIT_EMAIL
  {gCommonEventInfo[1], 2795, 3254},
  // category: activity_stream, method: event, object: DISCLAIMER_ACKED
  {gCommonEventInfo[1], 2795, 3267},
  // category: activity_stream, method: event, object: MENU_ADD_SEARCH
  {gCommonEventInfo[1], 2795, 3284},
  // category: activity_stream, method: event, object: MENU_ADD_TOPSITE
  {gCommonEventInfo[1], 2795, 3300},
  // category: activity_stream, method: event, object: MENU_COLLAPSE
  {gCommonEventInfo[1], 2795, 3317},
  // category: activity_stream, method: event, object: MENU_EXPAND
  {gCommonEventInfo[1], 2795, 3331},
  // category: activity_stream, method: event, object: MENU_MANAGE
  {gCommonEventInfo[1], 2795, 3343},
  // category: activity_stream, method: event, object: MENU_MOVE_DOWN
  {gCommonEventInfo[1], 2795, 3355},
  // category: activity_stream, method: event, object: MENU_MOVE_UP
  {gCommonEventInfo[1], 2795, 3370},
  // category: activity_stream, method: event, object: MENU_PRIVACY_NOTICE
  {gCommonEventInfo[1], 2795, 3383},
  // category: activity_stream, method: event, object: MENU_REMOVE
  {gCommonEventInfo[1], 2795, 3403},
  // category: activity_stream, method: event, object: TOP_SITES_EDIT
  {gCommonEventInfo[1], 2795, 3415},
  // category: activity_stream, method: event, object: TOP_SITES_EDIT_CLOSE
  {gCommonEventInfo[1], 2795, 3430},
  // category: activity_stream, method: event, object: TOPSITE_SPONSOR_INFO
  {gCommonEventInfo[1], 2795, 3451},
  // category: activity_stream, method: event, object: UNPIN
  {gCommonEventInfo[1], 2795, 3472},
  // category: addonsManager, method: action, object: aboutAddons
  {gCommonEventInfo[2], 57, 3478},
  // category: addonsManager, method: action, object: browserAction
  {gCommonEventInfo[2], 57, 3490},
  // category: addonsManager, method: action, object: customize
  {gCommonEventInfo[2], 57, 3504},
  // category: addonsManager, method: action, object: pageAction
  {gCommonEventInfo[2], 57, 3514},
  // category: addonsManager, method: action, object: doorhanger
  {gCommonEventInfo[2], 57, 3525},
  // category: addonsManager, method: action, object: appUpgrade
  {gCommonEventInfo[2], 57, 3536},
  // category: addonsManager, method: install, object: extension
  {gCommonEventInfo[3], 3547, 3555},
  // category: addonsManager, method: install, object: theme
  {gCommonEventInfo[3], 3547, 3565},
  // category: addonsManager, method: install, object: locale
  {gCommonEventInfo[3], 3547, 3571},
  // category: addonsManager, method: install, object: dictionary
  {gCommonEventInfo[3], 3547, 3578},
  // category: addonsManager, method: install, object: other
  {gCommonEventInfo[3], 3547, 3589},
  // category: addonsManager, method: install, object: unknown
  {gCommonEventInfo[3], 3547, 3595},
  // category: addonsManager, method: update, object: extension
  {gCommonEventInfo[3], 3603, 3555},
  // category: addonsManager, method: update, object: theme
  {gCommonEventInfo[3], 3603, 3565},
  // category: addonsManager, method: update, object: locale
  {gCommonEventInfo[3], 3603, 3571},
  // category: addonsManager, method: update, object: dictionary
  {gCommonEventInfo[3], 3603, 3578},
  // category: addonsManager, method: update, object: other
  {gCommonEventInfo[3], 3603, 3589},
  // category: addonsManager, method: update, object: unknown
  {gCommonEventInfo[3], 3603, 3595},
  // category: addonsManager, method: install_stats, object: extension
  {gCommonEventInfo[4], 3610, 3555},
  // category: addonsManager, method: install_stats, object: theme
  {gCommonEventInfo[4], 3610, 3565},
  // category: addonsManager, method: install_stats, object: locale
  {gCommonEventInfo[4], 3610, 3571},
  // category: addonsManager, method: install_stats, object: dictionary
  {gCommonEventInfo[4], 3610, 3578},
  // category: addonsManager, method: install_stats, object: other
  {gCommonEventInfo[4], 3610, 3589},
  // category: addonsManager, method: install_stats, object: unknown
  {gCommonEventInfo[4], 3610, 3595},
  // category: addonsManager, method: link, object: aboutAddons
  {gCommonEventInfo[5], 3624, 3478},
  // category: addonsManager, method: link, object: aboutPreferences
  {gCommonEventInfo[5], 3624, 3629},
  // category: addonsManager, method: link, object: customize
  {gCommonEventInfo[5], 3624, 3504},
  // category: addonsManager, method: disable, object: extension
  {gCommonEventInfo[6], 3646, 3555},
  // category: addonsManager, method: disable, object: theme
  {gCommonEventInfo[6], 3646, 3565},
  // category: addonsManager, method: disable, object: locale
  {gCommonEventInfo[6], 3646, 3571},
  // category: addonsManager, method: disable, object: dictionary
  {gCommonEventInfo[6], 3646, 3578},
  // category: addonsManager, method: disable, object: other
  {gCommonEventInfo[6], 3646, 3589},
  // category: addonsManager, method: enable, object: extension
  {gCommonEventInfo[6], 3654, 3555},
  // category: addonsManager, method: enable, object: theme
  {gCommonEventInfo[6], 3654, 3565},
  // category: addonsManager, method: enable, object: locale
  {gCommonEventInfo[6], 3654, 3571},
  // category: addonsManager, method: enable, object: dictionary
  {gCommonEventInfo[6], 3654, 3578},
  // category: addonsManager, method: enable, object: other
  {gCommonEventInfo[6], 3654, 3589},
  // category: addonsManager, method: sideload_prompt, object: extension
  {gCommonEventInfo[6], 3661, 3555},
  // category: addonsManager, method: sideload_prompt, object: theme
  {gCommonEventInfo[6], 3661, 3565},
  // category: addonsManager, method: sideload_prompt, object: locale
  {gCommonEventInfo[6], 3661, 3571},
  // category: addonsManager, method: sideload_prompt, object: dictionary
  {gCommonEventInfo[6], 3661, 3578},
  // category: addonsManager, method: sideload_prompt, object: other
  {gCommonEventInfo[6], 3661, 3589},
  // category: addonsManager, method: uninstall, object: extension
  {gCommonEventInfo[6], 3677, 3555},
  // category: addonsManager, method: uninstall, object: theme
  {gCommonEventInfo[6], 3677, 3565},
  // category: addonsManager, method: uninstall, object: locale
  {gCommonEventInfo[6], 3677, 3571},
  // category: addonsManager, method: uninstall, object: dictionary
  {gCommonEventInfo[6], 3677, 3578},
  // category: addonsManager, method: uninstall, object: other
  {gCommonEventInfo[6], 3677, 3589},
  // category: addonsManager, method: report, object: amo
  {gCommonEventInfo[7], 3687, 3694},
  // category: addonsManager, method: report, object: menu
  {gCommonEventInfo[7], 3687, 3698},
  // category: addonsManager, method: report, object: toolbar_context_menu
  {gCommonEventInfo[7], 3687, 3703},
  // category: addonsManager, method: report, object: uninstall
  {gCommonEventInfo[7], 3687, 3677},
  // category: addonsManager, method: view, object: aboutAddons
  {gCommonEventInfo[8], 77, 3478},
  // category: avif, method: dav1d_get_picture, object: return_value
  {gCommonEventInfo[9], 3724, 3742},
  // category: contextservices.quicksuggest, method: enable_toggled, object: enabled
  {gCommonEventInfo[10], 3755, 3770},
  // category: contextservices.quicksuggest, method: enable_toggled, object: disabled
  {gCommonEventInfo[10], 3755, 3778},
  // category: creditcard, method: detected, object: cc_form
  {gCommonEventInfo[11], 3787, 3796},
  // category: creditcard, method: popup_shown, object: cc_form
  {gCommonEventInfo[11], 3804, 3796},
  // category: creditcard, method: filled, object: cc_form
  {gCommonEventInfo[11], 1701, 3796},
  // category: creditcard, method: filled_modified, object: cc_form
  {gCommonEventInfo[11], 3816, 3796},
  // category: creditcard, method: submitted, object: cc_form
  {gCommonEventInfo[11], 3832, 3796},
  // category: creditcard, method: show, object: capture_doorhanger
  {gCommonEventInfo[12], 3842, 3847},
  // category: creditcard, method: show, object: update_doorhanger
  {gCommonEventInfo[12], 3842, 3866},
  // category: creditcard, method: save, object: capture_doorhanger
  {gCommonEventInfo[12], 3884, 3847},
  // category: creditcard, method: save, object: update_doorhanger
  {gCommonEventInfo[12], 3884, 3866},
  // category: creditcard, method: update, object: capture_doorhanger
  {gCommonEventInfo[12], 3603, 3847},
  // category: creditcard, method: update, object: update_doorhanger
  {gCommonEventInfo[12], 3603, 3866},
  // category: creditcard, method: cancel, object: capture_doorhanger
  {gCommonEventInfo[12], 3889, 3847},
  // category: creditcard, method: cancel, object: update_doorhanger
  {gCommonEventInfo[12], 3889, 3866},
  // category: creditcard, method: disable, object: capture_doorhanger
  {gCommonEventInfo[12], 3646, 3847},
  // category: creditcard, method: disable, object: update_doorhanger
  {gCommonEventInfo[12], 3646, 3866},
  // category: creditcard, method: show, object: manage
  {gCommonEventInfo[13], 3842, 3896},
  // category: creditcard, method: add, object: manage
  {gCommonEventInfo[13], 3903, 3896},
  // category: creditcard, method: delete, object: manage
  {gCommonEventInfo[13], 3907, 3896},
  // category: creditcard, method: show_entry, object: manage
  {gCommonEventInfo[13], 3914, 3896},
  // category: creditcard, method: edit, object: manage
  {gCommonEventInfo[13], 3925, 3896},
  // category: devtools.main, method: activate, object: responsive_design
  {gCommonEventInfo[14], 3930, 3939},
  // category: devtools.main, method: activate, object: split_console
  {gCommonEventInfo[14], 3930, 3957},
  // category: devtools.main, method: add_breakpoint, object: debugger
  {gCommonEventInfo[15], 3971, 3986},
  // category: devtools.main, method: blackbox, object: debugger
  {gCommonEventInfo[16], 3995, 3986},
  // category: devtools.main, method: close, object: tools
  {gCommonEventInfo[17], 4004, 4010},
  // category: devtools.main, method: close_adbg, object: aboutdebugging
  {gCommonEventInfo[18], 4016, 4027},
  // category: devtools.main, method: close_source_tab, object: debugger
  {gCommonEventInfo[19], 4042, 3986},
  // category: devtools.main, method: connection_attempt, object: aboutdebugging
  {gCommonEventInfo[20], 4059, 4027},
  // category: devtools.main, method: continue, object: debugger
  {gCommonEventInfo[21], 4078, 3986},
  // category: devtools.main, method: deactivate, object: responsive_design
  {gCommonEventInfo[22], 4087, 3939},
  // category: devtools.main, method: deactivate, object: split_console
  {gCommonEventInfo[22], 4087, 3957},
  // category: devtools.main, method: device_added, object: aboutdebugging
  {gCommonEventInfo[23], 4098, 4027},
  // category: devtools.main, method: device_removed, object: aboutdebugging
  {gCommonEventInfo[24], 4111, 4027},
  // category: devtools.main, method: edit_html, object: inspector
  {gCommonEventInfo[25], 4126, 4136},
  // category: devtools.main, method: edit_resend, object: netmonitor
  {gCommonEventInfo[26], 4146, 4158},
  // category: devtools.main, method: edit_rule, object: ruleview
  {gCommonEventInfo[27], 4169, 4179},
  // category: devtools.main, method: enter, object: accessibility
  {gCommonEventInfo[28], 4188, 4194},
  // category: devtools.main, method: enter, object: application
  {gCommonEventInfo[28], 4188, 4208},
  // category: devtools.main, method: enter, object: dom
  {gCommonEventInfo[28], 4188, 4220},
  // category: devtools.main, method: enter, object: inspector
  {gCommonEventInfo[28], 4188, 4136},
  // category: devtools.main, method: enter, object: jsdebugger
  {gCommonEventInfo[28], 4188, 4224},
  // category: devtools.main, method: enter, object: memory
  {gCommonEventInfo[28], 4188, 4235},
  // category: devtools.main, method: enter, object: netmonitor
  {gCommonEventInfo[28], 4188, 4158},
  // category: devtools.main, method: enter, object: options
  {gCommonEventInfo[28], 4188, 4242},
  // category: devtools.main, method: enter, object: performance
  {gCommonEventInfo[28], 4188, 4250},
  // category: devtools.main, method: enter, object: storage
  {gCommonEventInfo[28], 4188, 4262},
  // category: devtools.main, method: enter, object: styleeditor
  {gCommonEventInfo[28], 4188, 4270},
  // category: devtools.main, method: enter, object: webconsole
  {gCommonEventInfo[28], 4188, 4282},
  // category: devtools.main, method: enter, object: whatsnew
  {gCommonEventInfo[28], 4188, 4293},
  // category: devtools.main, method: enter, object: other
  {gCommonEventInfo[28], 4188, 3589},
  // category: devtools.main, method: enter, object: fakeTool4242
  {gCommonEventInfo[28], 4188, 4302},
  // category: devtools.main, method: enter, object: testBlankPanel
  {gCommonEventInfo[28], 4188, 4315},
  // category: devtools.main, method: enter, object: testTool
  {gCommonEventInfo[28], 4188, 4330},
  // category: devtools.main, method: enter, object: testtool1
  {gCommonEventInfo[28], 4188, 4339},
  // category: devtools.main, method: enter, object: testTool1072208
  {gCommonEventInfo[28], 4188, 4349},
  // category: devtools.main, method: enter, object: testtool2
  {gCommonEventInfo[28], 4188, 4365},
  // category: devtools.main, method: execute_js, object: webconsole
  {gCommonEventInfo[29], 4375, 4282},
  // category: devtools.main, method: exit, object: accessibility
  {gCommonEventInfo[30], 4386, 4194},
  // category: devtools.main, method: exit, object: application
  {gCommonEventInfo[30], 4386, 4208},
  // category: devtools.main, method: exit, object: dom
  {gCommonEventInfo[30], 4386, 4220},
  // category: devtools.main, method: exit, object: inspector
  {gCommonEventInfo[30], 4386, 4136},
  // category: devtools.main, method: exit, object: jsdebugger
  {gCommonEventInfo[30], 4386, 4224},
  // category: devtools.main, method: exit, object: memory
  {gCommonEventInfo[30], 4386, 4235},
  // category: devtools.main, method: exit, object: netmonitor
  {gCommonEventInfo[30], 4386, 4158},
  // category: devtools.main, method: exit, object: options
  {gCommonEventInfo[30], 4386, 4242},
  // category: devtools.main, method: exit, object: performance
  {gCommonEventInfo[30], 4386, 4250},
  // category: devtools.main, method: exit, object: storage
  {gCommonEventInfo[30], 4386, 4262},
  // category: devtools.main, method: exit, object: styleeditor
  {gCommonEventInfo[30], 4386, 4270},
  // category: devtools.main, method: exit, object: webconsole
  {gCommonEventInfo[30], 4386, 4282},
  // category: devtools.main, method: exit, object: whatsnew
  {gCommonEventInfo[30], 4386, 4293},
  // category: devtools.main, method: exit, object: other
  {gCommonEventInfo[30], 4386, 3589},
  // category: devtools.main, method: exit, object: fakeTool4242
  {gCommonEventInfo[30], 4386, 4302},
  // category: devtools.main, method: exit, object: testBlankPanel
  {gCommonEventInfo[30], 4386, 4315},
  // category: devtools.main, method: exit, object: testTool
  {gCommonEventInfo[30], 4386, 4330},
  // category: devtools.main, method: exit, object: testtool1
  {gCommonEventInfo[30], 4386, 4339},
  // category: devtools.main, method: exit, object: testTool1072208
  {gCommonEventInfo[30], 4386, 4349},
  // category: devtools.main, method: exit, object: testtool2
  {gCommonEventInfo[30], 4386, 4365},
  // category: devtools.main, method: f12_enabled, object: tools
  {gCommonEventInfo[31], 4391, 4010},
  // category: devtools.main, method: f12_popup_displayed, object: tools
  {gCommonEventInfo[32], 4403, 4010},
  // category: devtools.main, method: filters_changed, object: netmonitor
  {gCommonEventInfo[33], 4423, 4158},
  // category: devtools.main, method: filters_changed, object: webconsole
  {gCommonEventInfo[33], 4423, 4282},
  // category: devtools.main, method: inspect, object: aboutdebugging
  {gCommonEventInfo[34], 4439, 4027},
  // category: devtools.main, method: jump_to_definition, object: webconsole
  {gCommonEventInfo[35], 4447, 4282},
  // category: devtools.main, method: jump_to_source, object: webconsole
  {gCommonEventInfo[36], 4466, 4282},
  // category: devtools.main, method: object_expanded, object: webconsole
  {gCommonEventInfo[37], 4481, 4282},
  // category: devtools.main, method: open, object: tools
  {gCommonEventInfo[38], 4497, 4010},
  // category: devtools.main, method: open_adbg, object: aboutdebugging
  {gCommonEventInfo[39], 4502, 4027},
  // category: devtools.main, method: pause, object: debugger
  {gCommonEventInfo[40], 4512, 3986},
  // category: devtools.main, method: pause_on_exceptions, object: debugger
  {gCommonEventInfo[41], 4518, 3986},
  // category: devtools.main, method: persist_changed, object: netmonitor
  {gCommonEventInfo[42], 4538, 4158},
  // category: devtools.main, method: persist_changed, object: webconsole
  {gCommonEventInfo[42], 4538, 4282},
  // category: devtools.main, method: pretty_print, object: debugger
  {gCommonEventInfo[43], 4554, 3986},
  // category: devtools.main, method: remove_breakpoint, object: debugger
  {gCommonEventInfo[44], 4567, 3986},
  // category: devtools.main, method: reverse_search, object: webconsole
  {gCommonEventInfo[45], 4585, 4282},
  // category: devtools.main, method: runtime_added, object: aboutdebugging
  {gCommonEventInfo[46], 4600, 4027},
  // category: devtools.main, method: runtime_connected, object: aboutdebugging
  {gCommonEventInfo[47], 4614, 4027},
  // category: devtools.main, method: runtime_disconnected, object: aboutdebugging
  {gCommonEventInfo[48], 4632, 4027},
  // category: devtools.main, method: runtime_removed, object: aboutdebugging
  {gCommonEventInfo[49], 4653, 4027},
  // category: devtools.main, method: select_page, object: aboutdebugging
  {gCommonEventInfo[50], 4669, 4027},
  // category: devtools.main, method: select_page, object: application
  {gCommonEventInfo[50], 4669, 4208},
  // category: devtools.main, method: select_ws_frame, object: netmonitor
  {gCommonEventInfo[51], 4681, 4158},
  // category: devtools.main, method: show_profiler, object: aboutdebugging
  {gCommonEventInfo[52], 4697, 4027},
  // category: devtools.main, method: sidepanel_changed, object: inspector
  {gCommonEventInfo[53], 4711, 4136},
  // category: devtools.main, method: sidepanel_changed, object: netmonitor
  {gCommonEventInfo[53], 4711, 4158},
  // category: devtools.main, method: start_worker, object: application
  {gCommonEventInfo[54], 4729, 4208},
  // category: devtools.main, method: throttle_changed, object: netmonitor
  {gCommonEventInfo[55], 4742, 4158},
  // category: devtools.main, method: tool_timer, object: animationinspector
  {gCommonEventInfo[56], 4759, 4770},
  // category: devtools.main, method: tool_timer, object: compatibilityview
  {gCommonEventInfo[56], 4759, 4789},
  // category: devtools.main, method: tool_timer, object: computedview
  {gCommonEventInfo[56], 4759, 4807},
  // category: devtools.main, method: tool_timer, object: changesview
  {gCommonEventInfo[56], 4759, 4820},
  // category: devtools.main, method: tool_timer, object: fontinspector
  {gCommonEventInfo[56], 4759, 4832},
  // category: devtools.main, method: tool_timer, object: layoutview
  {gCommonEventInfo[56], 4759, 4846},
  // category: devtools.main, method: tool_timer, object: ruleview
  {gCommonEventInfo[56], 4759, 4179},
  // category: devtools.main, method: unregister_worker, object: application
  {gCommonEventInfo[57], 4857, 4208},
  // category: devtools.main, method: update_conn_prompt, object: aboutdebugging
  {gCommonEventInfo[58], 4875, 4027},
  // category: doh, method: evaluate, object: heuristics
  {gCommonEventInfo[59], 4894, 4903},
  // category: doh, method: evaluate_v2, object: heuristics
  {gCommonEventInfo[60], 4914, 4903},
  // category: doh, method: state, object: enabled
  {gCommonEventInfo[61], 4926, 3770},
  // category: doh, method: state, object: disabled
  {gCommonEventInfo[61], 4926, 3778},
  // category: doh, method: state, object: manuallyDisabled
  {gCommonEventInfo[61], 4926, 4932},
  // category: doh, method: state, object: policyDisabled
  {gCommonEventInfo[61], 4926, 4949},
  // category: doh, method: state, object: uninstalled
  {gCommonEventInfo[61], 4926, 4964},
  // category: doh, method: state, object: UIOk
  {gCommonEventInfo[61], 4926, 4976},
  // category: doh, method: state, object: UIDisabled
  {gCommonEventInfo[61], 4926, 4981},
  // category: doh, method: state, object: rollback
  {gCommonEventInfo[61], 4926, 4992},
  // category: doh, method: state, object: shutdown
  {gCommonEventInfo[61], 4926, 5001},
  // category: dom.quota.try, method: error, object: step
  {gCommonEventInfo[62], 105, 137},
  // category: downloads, method: added, object: fileExtension
  {gCommonEventInfo[63], 5010, 5016},
  // category: extensions.data, method: migrateResult, object: storageLocal
  {gCommonEventInfo[64], 5030, 5044},
  // category: extensions.data, method: storageLocalError, object: get
  {gCommonEventInfo[65], 5057, 5075},
  // category: extensions.data, method: storageLocalError, object: set
  {gCommonEventInfo[65], 5057, 5079},
  // category: extensions.data, method: storageLocalError, object: remove
  {gCommonEventInfo[65], 5057, 5083},
  // category: extensions.data, method: storageLocalError, object: clear
  {gCommonEventInfo[65], 5057, 5090},
  // category: form_autocomplete, method: show, object: logins
  {gCommonEventInfo[66], 3842, 5096},
  // category: fxa, method: connect, object: account
  {gCommonEventInfo[67], 5103, 5111},
  // category: fxa, method: disconnect, object: account
  {gCommonEventInfo[67], 5119, 5111},
  // category: fxa_app_menu, method: click, object: account_settings
  {gCommonEventInfo[68], 5130, 5136},
  // category: fxa_app_menu, method: click, object: cad
  {gCommonEventInfo[68], 5130, 5153},
  // category: fxa_app_menu, method: click, object: login
  {gCommonEventInfo[68], 5130, 1129},
  // category: fxa_app_menu, method: click, object: send_tab
  {gCommonEventInfo[68], 5130, 5157},
  // category: fxa_app_menu, method: click, object: sync_now
  {gCommonEventInfo[68], 5130, 5166},
  // category: fxa_app_menu, method: click, object: sync_settings
  {gCommonEventInfo[68], 5130, 5175},
  // category: fxa_app_menu, method: click, object: sync_tabs
  {gCommonEventInfo[68], 5130, 5189},
  // category: fxa_app_menu, method: click, object: sync_tabs_sidebar
  {gCommonEventInfo[68], 5130, 5199},
  // category: fxa_app_menu, method: click, object: toolbar_icon
  {gCommonEventInfo[68], 5130, 5217},
  // category: fxa_app_menu, method: click, object: unver_sync_settings
  {gCommonEventInfo[68], 5130, 5230},
  // category: fxa_app_menu, method: click, object: open_monitor
  {gCommonEventInfo[68], 5130, 5250},
  // category: fxa_app_menu, method: click, object: open_send
  {gCommonEventInfo[68], 5130, 5263},
  // category: fxa_avatar_menu, method: click, object: account_settings
  {gCommonEventInfo[69], 5130, 5136},
  // category: fxa_avatar_menu, method: click, object: cad
  {gCommonEventInfo[69], 5130, 5153},
  // category: fxa_avatar_menu, method: click, object: login
  {gCommonEventInfo[69], 5130, 1129},
  // category: fxa_avatar_menu, method: click, object: send_tab
  {gCommonEventInfo[69], 5130, 5157},
  // category: fxa_avatar_menu, method: click, object: sync_now
  {gCommonEventInfo[69], 5130, 5166},
  // category: fxa_avatar_menu, method: click, object: sync_settings
  {gCommonEventInfo[69], 5130, 5175},
  // category: fxa_avatar_menu, method: click, object: sync_tabs
  {gCommonEventInfo[69], 5130, 5189},
  // category: fxa_avatar_menu, method: click, object: sync_tabs_sidebar
  {gCommonEventInfo[69], 5130, 5199},
  // category: fxa_avatar_menu, method: click, object: toolbar_icon
  {gCommonEventInfo[69], 5130, 5217},
  // category: fxa_avatar_menu, method: click, object: unver_sync_settings
  {gCommonEventInfo[69], 5130, 5230},
  // category: fxa_avatar_menu, method: click, object: open_monitor
  {gCommonEventInfo[69], 5130, 5250},
  // category: fxa_avatar_menu, method: click, object: open_send
  {gCommonEventInfo[69], 5130, 5263},
  // category: homepage, method: preference, object: ignore
  {gCommonEventInfo[70], 1477, 5273},
  // category: installation, method: first_seen, object: full
  {gCommonEventInfo[71], 5280, 5291},
  // category: installation, method: first_seen, object: stub
  {gCommonEventInfo[71], 5280, 5296},
  // category: intl.ui.browserLanguage, method: manage, object: dialog
  {gCommonEventInfo[72], 3896, 5301},
  // category: intl.ui.browserLanguage, method: manage, object: main
  {gCommonEventInfo[72], 3896, 5308},
  // category: intl.ui.browserLanguage, method: search, object: dialog
  {gCommonEventInfo[72], 5313, 5301},
  // category: intl.ui.browserLanguage, method: search, object: main
  {gCommonEventInfo[72], 5313, 5308},
  // category: intl.ui.browserLanguage, method: add, object: dialog
  {gCommonEventInfo[72], 3903, 5301},
  // category: intl.ui.browserLanguage, method: add, object: main
  {gCommonEventInfo[72], 3903, 5308},
  // category: intl.ui.browserLanguage, method: remove, object: dialog
  {gCommonEventInfo[72], 5083, 5301},
  // category: intl.ui.browserLanguage, method: remove, object: main
  {gCommonEventInfo[72], 5083, 5308},
  // category: intl.ui.browserLanguage, method: reorder, object: dialog
  {gCommonEventInfo[72], 5320, 5301},
  // category: intl.ui.browserLanguage, method: reorder, object: main
  {gCommonEventInfo[72], 5320, 5308},
  // category: intl.ui.browserLanguage, method: apply, object: dialog
  {gCommonEventInfo[72], 5328, 5301},
  // category: intl.ui.browserLanguage, method: apply, object: main
  {gCommonEventInfo[72], 5328, 5308},
  // category: intl.ui.browserLanguage, method: accept, object: dialog
  {gCommonEventInfo[72], 5334, 5301},
  // category: intl.ui.browserLanguage, method: accept, object: main
  {gCommonEventInfo[72], 5334, 5308},
  // category: intl.ui.browserLanguage, method: cancel, object: dialog
  {gCommonEventInfo[72], 3889, 5301},
  // category: intl.ui.browserLanguage, method: cancel, object: main
  {gCommonEventInfo[72], 3889, 5308},
  // category: jsonfile, method: load, object: logins
  {gCommonEventInfo[73], 5341, 5096},
  // category: jsonfile, method: load, object: autofillprofiles
  {gCommonEventInfo[73], 5341, 5346},
  // category: messaging_experiments, method: reach, object: cfr
  {gCommonEventInfo[74], 5363, 5369},
  // category: messaging_experiments, method: reach, object: whats_new_panel
  {gCommonEventInfo[74], 5363, 5373},
  // category: messaging_experiments, method: reach, object: moments_page
  {gCommonEventInfo[74], 5363, 5389},
  // category: messaging_experiments, method: reach, object: snippets
  {gCommonEventInfo[74], 5363, 5402},
  // category: messaging_experiments, method: reach, object: cfr_fxa
  {gCommonEventInfo[74], 5363, 5411},
  // category: messaging_experiments, method: targeting, object: attribute_error
  {gCommonEventInfo[75], 5419, 5429},
  // category: messaging_experiments, method: targeting, object: attribute_timeout
  {gCommonEventInfo[75], 5419, 5445},
  // category: navigation, method: search, object: about_home
  {gCommonEventInfo[76], 5313, 5463},
  // category: navigation, method: search, object: about_newtab
  {gCommonEventInfo[76], 5313, 5474},
  // category: navigation, method: search, object: contextmenu
  {gCommonEventInfo[76], 5313, 5487},
  // category: navigation, method: search, object: oneoff
  {gCommonEventInfo[76], 5313, 5499},
  // category: navigation, method: search, object: suggestion
  {gCommonEventInfo[76], 5313, 5506},
  // category: navigation, method: search, object: alias
  {gCommonEventInfo[76], 5313, 5517},
  // category: navigation, method: search, object: enter
  {gCommonEventInfo[76], 5313, 4188},
  // category: navigation, method: search, object: searchbar
  {gCommonEventInfo[76], 5313, 5523},
  // category: navigation, method: search, object: urlbar
  {gCommonEventInfo[76], 5313, 2732},
  // category: navigation, method: search, object: urlbar_searchmode
  {gCommonEventInfo[76], 5313, 5533},
  // category: navigation, method: search, object: webextension
  {gCommonEventInfo[76], 5313, 5551},
  // category: network.dns, method: trrConfirmation, object: context
  {gCommonEventInfo[77], 5564, 939},
  // category: normandy, method: enroll, object: preference_study
  {gCommonEventInfo[78], 5580, 5587},
  // category: normandy, method: enroll, object: addon_study
  {gCommonEventInfo[78], 5580, 5604},
  // category: normandy, method: enroll, object: preference_rollout
  {gCommonEventInfo[78], 5580, 5616},
  // category: normandy, method: enroll, object: addon_rollout
  {gCommonEventInfo[78], 5580, 5635},
  // category: normandy, method: enrollFailed, object: addon_study
  {gCommonEventInfo[79], 5649, 5604},
  // category: normandy, method: enrollFailed, object: preference_rollout
  {gCommonEventInfo[79], 5649, 5616},
  // category: normandy, method: enrollFailed, object: preference_study
  {gCommonEventInfo[79], 5649, 5587},
  // category: normandy, method: enrollFailed, object: addon_rollout
  {gCommonEventInfo[79], 5649, 5635},
  // category: normandy, method: expose, object: feature_study
  {gCommonEventInfo[80], 5662, 5669},
  // category: normandy, method: graduate, object: preference_rollout
  {gCommonEventInfo[81], 5683, 5616},
  // category: normandy, method: unenroll, object: preference_study
  {gCommonEventInfo[82], 5692, 5587},
  // category: normandy, method: unenroll, object: addon_study
  {gCommonEventInfo[82], 5692, 5604},
  // category: normandy, method: unenroll, object: preference_rollback
  {gCommonEventInfo[82], 5692, 5701},
  // category: normandy, method: unenroll, object: addon_rollback
  {gCommonEventInfo[82], 5692, 5721},
  // category: normandy, method: unenrollFailed, object: preference_rollback
  {gCommonEventInfo[83], 5736, 5701},
  // category: normandy, method: unenrollFailed, object: preference_study
  {gCommonEventInfo[83], 5736, 5587},
  // category: normandy, method: unenrollFailed, object: addon_rollback
  {gCommonEventInfo[83], 5736, 5721},
  // category: normandy, method: update, object: addon_study
  {gCommonEventInfo[84], 3603, 5604},
  // category: normandy, method: update, object: preference_rollout
  {gCommonEventInfo[84], 3603, 5616},
  // category: normandy, method: update, object: addon_rollout
  {gCommonEventInfo[84], 3603, 5635},
  // category: normandy, method: updateFailed, object: addon_study
  {gCommonEventInfo[85], 5751, 5604},
  // category: normandy, method: updateFailed, object: addon_rollout
  {gCommonEventInfo[85], 5751, 5635},
  // category: partner_link, method: attribution, object: success
  {gCommonEventInfo[86], 5764, 5776},
  // category: partner_link, method: attribution, object: failure
  {gCommonEventInfo[86], 5764, 5784},
  // category: partner_link, method: attribution, object: abort
  {gCommonEventInfo[86], 5764, 5792},
  // category: partner_link, method: click, object: newtab
  {gCommonEventInfo[87], 5130, 5798},
  // category: partner_link, method: click, object: urlbar
  {gCommonEventInfo[87], 5130, 2732},
  // category: pictureinpicture, method: create, object: player
  {gCommonEventInfo[88], 5805, 5812},
  // category: pictureinpicture, method: move, object: player
  {gCommonEventInfo[89], 5819, 5812},
  // category: pictureinpicture, method: resize, object: player
  {gCommonEventInfo[90], 5824, 5812},
  // category: pwmgr, method: autocomplete_field, object: generatedpassword
  {gCommonEventInfo[91], 5831, 5850},
  // category: pwmgr, method: autocomplete_shown, object: generatedpassword
  {gCommonEventInfo[91], 5868, 5850},
  // category: pwmgr, method: doorhanger_submitted, object: save
  {gCommonEventInfo[92], 5887, 3884},
  // category: pwmgr, method: doorhanger_submitted, object: update
  {gCommonEventInfo[92], 5887, 3603},
  // category: pwmgr, method: filled_field_edited, object: generatedpassword
  {gCommonEventInfo[93], 5908, 5850},
  // category: pwmgr, method: cancel, object: existing_login
  {gCommonEventInfo[94], 3889, 5928},
  // category: pwmgr, method: cancel, object: list
  {gCommonEventInfo[94], 3889, 5943},
  // category: pwmgr, method: cancel, object: new_login
  {gCommonEventInfo[94], 3889, 5948},
  // category: pwmgr, method: cancel, object: password
  {gCommonEventInfo[94], 3889, 5958},
  // category: pwmgr, method: cancel, object: username
  {gCommonEventInfo[94], 3889, 5967},
  // category: pwmgr, method: copy, object: existing_login
  {gCommonEventInfo[94], 5976, 5928},
  // category: pwmgr, method: copy, object: list
  {gCommonEventInfo[94], 5976, 5943},
  // category: pwmgr, method: copy, object: new_login
  {gCommonEventInfo[94], 5976, 5948},
  // category: pwmgr, method: copy, object: password
  {gCommonEventInfo[94], 5976, 5958},
  // category: pwmgr, method: copy, object: username
  {gCommonEventInfo[94], 5976, 5967},
  // category: pwmgr, method: delete, object: existing_login
  {gCommonEventInfo[94], 3907, 5928},
  // category: pwmgr, method: delete, object: list
  {gCommonEventInfo[94], 3907, 5943},
  // category: pwmgr, method: delete, object: new_login
  {gCommonEventInfo[94], 3907, 5948},
  // category: pwmgr, method: delete, object: password
  {gCommonEventInfo[94], 3907, 5958},
  // category: pwmgr, method: delete, object: username
  {gCommonEventInfo[94], 3907, 5967},
  // category: pwmgr, method: dismiss_breach_alert, object: existing_login
  {gCommonEventInfo[94], 5981, 5928},
  // category: pwmgr, method: dismiss_breach_alert, object: list
  {gCommonEventInfo[94], 5981, 5943},
  // category: pwmgr, method: dismiss_breach_alert, object: new_login
  {gCommonEventInfo[94], 5981, 5948},
  // category: pwmgr, method: dismiss_breach_alert, object: password
  {gCommonEventInfo[94], 5981, 5958},
  // category: pwmgr, method: dismiss_breach_alert, object: username
  {gCommonEventInfo[94], 5981, 5967},
  // category: pwmgr, method: edit, object: existing_login
  {gCommonEventInfo[94], 3925, 5928},
  // category: pwmgr, method: edit, object: list
  {gCommonEventInfo[94], 3925, 5943},
  // category: pwmgr, method: edit, object: new_login
  {gCommonEventInfo[94], 3925, 5948},
  // category: pwmgr, method: edit, object: password
  {gCommonEventInfo[94], 3925, 5958},
  // category: pwmgr, method: edit, object: username
  {gCommonEventInfo[94], 3925, 5967},
  // category: pwmgr, method: filter, object: existing_login
  {gCommonEventInfo[94], 6002, 5928},
  // category: pwmgr, method: filter, object: list
  {gCommonEventInfo[94], 6002, 5943},
  // category: pwmgr, method: filter, object: new_login
  {gCommonEventInfo[94], 6002, 5948},
  // category: pwmgr, method: filter, object: password
  {gCommonEventInfo[94], 6002, 5958},
  // category: pwmgr, method: filter, object: username
  {gCommonEventInfo[94], 6002, 5967},
  // category: pwmgr, method: hide, object: existing_login
  {gCommonEventInfo[94], 6009, 5928},
  // category: pwmgr, method: hide, object: list
  {gCommonEventInfo[94], 6009, 5943},
  // category: pwmgr, method: hide, object: new_login
  {gCommonEventInfo[94], 6009, 5948},
  // category: pwmgr, method: hide, object: password
  {gCommonEventInfo[94], 6009, 5958},
  // category: pwmgr, method: hide, object: username
  {gCommonEventInfo[94], 6009, 5967},
  // category: pwmgr, method: learn_more_breach, object: existing_login
  {gCommonEventInfo[94], 6014, 5928},
  // category: pwmgr, method: learn_more_breach, object: list
  {gCommonEventInfo[94], 6014, 5943},
  // category: pwmgr, method: learn_more_breach, object: new_login
  {gCommonEventInfo[94], 6014, 5948},
  // category: pwmgr, method: learn_more_breach, object: password
  {gCommonEventInfo[94], 6014, 5958},
  // category: pwmgr, method: learn_more_breach, object: username
  {gCommonEventInfo[94], 6014, 5967},
  // category: pwmgr, method: learn_more_vuln, object: existing_login
  {gCommonEventInfo[94], 6032, 5928},
  // category: pwmgr, method: learn_more_vuln, object: list
  {gCommonEventInfo[94], 6032, 5943},
  // category: pwmgr, method: learn_more_vuln, object: new_login
  {gCommonEventInfo[94], 6032, 5948},
  // category: pwmgr, method: learn_more_vuln, object: password
  {gCommonEventInfo[94], 6032, 5958},
  // category: pwmgr, method: learn_more_vuln, object: username
  {gCommonEventInfo[94], 6032, 5967},
  // category: pwmgr, method: new, object: existing_login
  {gCommonEventInfo[94], 6048, 5928},
  // category: pwmgr, method: new, object: list
  {gCommonEventInfo[94], 6048, 5943},
  // category: pwmgr, method: new, object: new_login
  {gCommonEventInfo[94], 6048, 5948},
  // category: pwmgr, method: new, object: password
  {gCommonEventInfo[94], 6048, 5958},
  // category: pwmgr, method: new, object: username
  {gCommonEventInfo[94], 6048, 5967},
  // category: pwmgr, method: open_site, object: existing_login
  {gCommonEventInfo[94], 6052, 5928},
  // category: pwmgr, method: open_site, object: list
  {gCommonEventInfo[94], 6052, 5943},
  // category: pwmgr, method: open_site, object: new_login
  {gCommonEventInfo[94], 6052, 5948},
  // category: pwmgr, method: open_site, object: password
  {gCommonEventInfo[94], 6052, 5958},
  // category: pwmgr, method: open_site, object: username
  {gCommonEventInfo[94], 6052, 5967},
  // category: pwmgr, method: save, object: existing_login
  {gCommonEventInfo[94], 3884, 5928},
  // category: pwmgr, method: save, object: list
  {gCommonEventInfo[94], 3884, 5943},
  // category: pwmgr, method: save, object: new_login
  {gCommonEventInfo[94], 3884, 5948},
  // category: pwmgr, method: save, object: password
  {gCommonEventInfo[94], 3884, 5958},
  // category: pwmgr, method: save, object: username
  {gCommonEventInfo[94], 3884, 5967},
  // category: pwmgr, method: select, object: existing_login
  {gCommonEventInfo[94], 6062, 5928},
  // category: pwmgr, method: select, object: list
  {gCommonEventInfo[94], 6062, 5943},
  // category: pwmgr, method: select, object: new_login
  {gCommonEventInfo[94], 6062, 5948},
  // category: pwmgr, method: select, object: password
  {gCommonEventInfo[94], 6062, 5958},
  // category: pwmgr, method: select, object: username
  {gCommonEventInfo[94], 6062, 5967},
  // category: pwmgr, method: show, object: existing_login
  {gCommonEventInfo[94], 3842, 5928},
  // category: pwmgr, method: show, object: list
  {gCommonEventInfo[94], 3842, 5943},
  // category: pwmgr, method: show, object: new_login
  {gCommonEventInfo[94], 3842, 5948},
  // category: pwmgr, method: show, object: password
  {gCommonEventInfo[94], 3842, 5958},
  // category: pwmgr, method: show, object: username
  {gCommonEventInfo[94], 3842, 5967},
  // category: pwmgr, method: sort, object: existing_login
  {gCommonEventInfo[94], 6069, 5928},
  // category: pwmgr, method: sort, object: list
  {gCommonEventInfo[94], 6069, 5943},
  // category: pwmgr, method: sort, object: new_login
  {gCommonEventInfo[94], 6069, 5948},
  // category: pwmgr, method: sort, object: password
  {gCommonEventInfo[94], 6069, 5958},
  // category: pwmgr, method: sort, object: username
  {gCommonEventInfo[94], 6069, 5967},
  // category: pwmgr, method: mgmt_menu_item_used, object: import_from_browser
  {gCommonEventInfo[95], 6074, 6094},
  // category: pwmgr, method: mgmt_menu_item_used, object: import_from_csv
  {gCommonEventInfo[95], 6074, 6114},
  // category: pwmgr, method: mgmt_menu_item_used, object: import_csv_complete
  {gCommonEventInfo[95], 6074, 6130},
  // category: pwmgr, method: mgmt_menu_item_used, object: export
  {gCommonEventInfo[95], 6074, 6150},
  // category: pwmgr, method: mgmt_menu_item_used, object: export_complete
  {gCommonEventInfo[95], 6074, 6157},
  // category: pwmgr, method: mgmt_menu_item_used, object: preferences
  {gCommonEventInfo[95], 6074, 6173},
  // category: pwmgr, method: open_management, object: aboutprotections
  {gCommonEventInfo[96], 6185, 6201},
  // category: pwmgr, method: open_management, object: autocomplete
  {gCommonEventInfo[96], 6185, 6218},
  // category: pwmgr, method: open_management, object: capturedoorhanger
  {gCommonEventInfo[96], 6185, 6231},
  // category: pwmgr, method: open_management, object: contextmenu
  {gCommonEventInfo[96], 6185, 5487},
  // category: pwmgr, method: open_management, object: direct
  {gCommonEventInfo[96], 6185, 6249},
  // category: pwmgr, method: open_management, object: fxamenu
  {gCommonEventInfo[96], 6185, 6256},
  // category: pwmgr, method: open_management, object: mainmenu
  {gCommonEventInfo[96], 6185, 6264},
  // category: pwmgr, method: open_management, object: pageinfo
  {gCommonEventInfo[96], 6185, 6273},
  // category: pwmgr, method: open_management, object: preferences
  {gCommonEventInfo[96], 6185, 6173},
  // category: pwmgr, method: open_management, object: snippet
  {gCommonEventInfo[96], 6185, 6282},
  // category: pwmgr, method: reauthenticate, object: master_password
  {gCommonEventInfo[97], 6290, 6305},
  // category: pwmgr, method: reauthenticate, object: os_auth
  {gCommonEventInfo[97], 6290, 6321},
  // category: pwmgr, method: saved_login_used, object: form_login
  {gCommonEventInfo[98], 6329, 6346},
  // category: pwmgr, method: saved_login_used, object: form_password
  {gCommonEventInfo[98], 6329, 6357},
  // category: pwmgr, method: saved_login_used, object: auth_login
  {gCommonEventInfo[98], 6329, 6371},
  // category: pwmgr, method: saved_login_used, object: prompt_login
  {gCommonEventInfo[98], 6329, 6382},
  // category: readermode, method: view, object: on
  {gCommonEventInfo[99], 77, 6395},
  // category: readermode, method: view, object: off
  {gCommonEventInfo[99], 77, 6398},
  // category: security, method: evalUsage, object: systemContext
  {gCommonEventInfo[100], 6402, 6412},
  // category: security, method: evalUsage, object: parentProcess
  {gCommonEventInfo[100], 6402, 6426},
  // category: security, method: fissionPrincipals, object: contentParent
  {gCommonEventInfo[101], 6440, 6458},
  // category: security, method: javascriptLoad, object: parentProcess
  {gCommonEventInfo[102], 6472, 6426},
  // category: security, method: unexpectedload, object: systemprincipal
  {gCommonEventInfo[103], 6487, 6502},
  // category: security.doh.trrPerformance, method: resolved, object: record
  {gCommonEventInfo[104], 6518, 6527},
  // category: security.doh.trrPerformance, method: trrselect, object: dryrunresult
  {gCommonEventInfo[105], 6534, 6544},
  // category: security.ui.app_menu, method: click, object: open_full_report
  {gCommonEventInfo[106], 5130, 6557},
  // category: security.ui.certerror, method: click, object: advanced_button
  {gCommonEventInfo[107], 5130, 6574},
  // category: security.ui.certerror, method: click, object: exception_button
  {gCommonEventInfo[107], 5130, 6590},
  // category: security.ui.certerror, method: click, object: return_button_top
  {gCommonEventInfo[107], 5130, 6607},
  // category: security.ui.certerror, method: click, object: return_button_adv
  {gCommonEventInfo[107], 5130, 6625},
  // category: security.ui.certerror, method: click, object: learn_more_link
  {gCommonEventInfo[107], 5130, 6643},
  // category: security.ui.certerror, method: click, object: auto_report_cb
  {gCommonEventInfo[107], 5130, 6659},
  // category: security.ui.certerror, method: click, object: error_code_link
  {gCommonEventInfo[107], 5130, 6674},
  // category: security.ui.certerror, method: click, object: clipboard_button_top
  {gCommonEventInfo[107], 5130, 6690},
  // category: security.ui.certerror, method: click, object: clipboard_button_bot
  {gCommonEventInfo[107], 5130, 6711},
  // category: security.ui.certerror, method: load, object: aboutcerterror
  {gCommonEventInfo[108], 5341, 6732},
  // category: security.ui.protections, method: click, object: lw_open_button
  {gCommonEventInfo[109], 5130, 6747},
  // category: security.ui.protections, method: click, object: lw_sync_link
  {gCommonEventInfo[109], 5130, 6762},
  // category: security.ui.protections, method: click, object: lw_about_link
  {gCommonEventInfo[109], 5130, 6775},
  // category: security.ui.protections, method: click, object: mtr_about_link
  {gCommonEventInfo[109], 5130, 6789},
  // category: security.ui.protections, method: click, object: mtr_report_link
  {gCommonEventInfo[109], 5130, 6804},
  // category: security.ui.protections, method: click, object: mtr_signup_button
  {gCommonEventInfo[109], 5130, 6820},
  // category: security.ui.protections, method: click, object: trackers_about_link
  {gCommonEventInfo[109], 5130, 6838},
  // category: security.ui.protections, method: click, object: mobile_app_link
  {gCommonEventInfo[109], 5130, 6858},
  // category: security.ui.protections, method: click, object: settings_link
  {gCommonEventInfo[109], 5130, 6874},
  // category: security.ui.protections, method: click, object: vpn_banner_link
  {gCommonEventInfo[109], 5130, 6888},
  // category: security.ui.protections, method: click, object: vpn_banner_close
  {gCommonEventInfo[109], 5130, 6904},
  // category: security.ui.protections, method: click, object: vpn_card_link
  {gCommonEventInfo[109], 5130, 6921},
  // category: security.ui.protections, method: click, object: vpn_app_link_android
  {gCommonEventInfo[109], 5130, 6935},
  // category: security.ui.protections, method: click, object: vpn_app_link_ios
  {gCommonEventInfo[109], 5130, 6956},
  // category: security.ui.protections, method: close, object: protection_report
  {gCommonEventInfo[110], 4004, 6973},
  // category: security.ui.protections, method: show, object: protection_report
  {gCommonEventInfo[111], 3842, 6973},
  // category: security.ui.protections, method: show, object: vpn_banner
  {gCommonEventInfo[111], 3842, 6991},
  // category: security.ui.protectionspopup, method: click, object: etp_toggle_on
  {gCommonEventInfo[112], 5130, 7002},
  // category: security.ui.protectionspopup, method: click, object: etp_toggle_off
  {gCommonEventInfo[112], 5130, 7016},
  // category: security.ui.protectionspopup, method: click, object: sitenotworking_link
  {gCommonEventInfo[112], 5130, 7031},
  // category: security.ui.protectionspopup, method: click, object: send_report_link
  {gCommonEventInfo[112], 5130, 7051},
  // category: security.ui.protectionspopup, method: click, object: send_report_submit
  {gCommonEventInfo[112], 5130, 7068},
  // category: security.ui.protectionspopup, method: click, object: social
  {gCommonEventInfo[112], 5130, 7087},
  // category: security.ui.protectionspopup, method: click, object: cookies
  {gCommonEventInfo[112], 5130, 7094},
  // category: security.ui.protectionspopup, method: click, object: trackers
  {gCommonEventInfo[112], 5130, 7102},
  // category: security.ui.protectionspopup, method: click, object: fingerprinters
  {gCommonEventInfo[112], 5130, 7111},
  // category: security.ui.protectionspopup, method: click, object: cryptominers
  {gCommonEventInfo[112], 5130, 7126},
  // category: security.ui.protectionspopup, method: click, object: subview_settings
  {gCommonEventInfo[112], 5130, 7139},
  // category: security.ui.protectionspopup, method: click, object: settings
  {gCommonEventInfo[112], 5130, 7156},
  // category: security.ui.protectionspopup, method: click, object: full_report
  {gCommonEventInfo[112], 5130, 7165},
  // category: security.ui.protectionspopup, method: click, object: milestone_message
  {gCommonEventInfo[112], 5130, 7177},
  // category: security.ui.protectionspopup, method: open, object: protections_popup
  {gCommonEventInfo[113], 4497, 7195},
  // category: slow_script_warning, method: shown, object: browser
  {gCommonEventInfo[114], 7213, 7219},
  // category: slow_script_warning, method: shown, object: content
  {gCommonEventInfo[114], 7213, 7227},
  // category: telemetry.test, method: content_only, object: object1
  {gCommonEventInfo[115], 7235, 7248},
  // category: telemetry.test, method: default_products, object: object1
  {gCommonEventInfo[116], 7256, 7248},
  // category: telemetry.test, method: desktop_only, object: object1
  {gCommonEventInfo[117], 7273, 7248},
  // category: telemetry.test, method: expired_version, object: object1
  {gCommonEventInfo[118], 7286, 7248},
  // category: telemetry.test, method: expired_version, object: object2
  {gCommonEventInfo[118], 7286, 7302},
  // category: telemetry.test, method: main_and_content, object: object1
  {gCommonEventInfo[119], 7310, 7248},
  // category: telemetry.test, method: main_only, object: object1
  {gCommonEventInfo[120], 7327, 7248},
  // category: telemetry.test, method: mobile_only, object: object1
  {gCommonEventInfo[121], 7337, 7248},
  // category: telemetry.test, method: multiproduct, object: object1
  {gCommonEventInfo[122], 7349, 7248},
  // category: telemetry.test, method: not_expired_optout, object: object1
  {gCommonEventInfo[123], 7362, 7248},
  // category: telemetry.test, method: optout, object: object1
  {gCommonEventInfo[124], 7381, 7248},
  // category: telemetry.test, method: optout, object: object2
  {gCommonEventInfo[124], 7381, 7302},
  // category: telemetry.test, method: test1, object: object1
  {gCommonEventInfo[125], 7388, 7248},
  // category: telemetry.test, method: test1, object: object2
  {gCommonEventInfo[125], 7388, 7302},
  // category: telemetry.test, method: test2, object: object1
  {gCommonEventInfo[125], 7394, 7248},
  // category: telemetry.test, method: test2, object: object2
  {gCommonEventInfo[125], 7394, 7302},
  // category: telemetry.test.second, method: test, object: object1
  {gCommonEventInfo[126], 7400, 7248},
  // category: telemetry.test.second, method: test, object: object2
  {gCommonEventInfo[126], 7400, 7302},
  // category: telemetry.test.second, method: test, object: object3
  {gCommonEventInfo[126], 7400, 7405},
  // category: uptake.remotecontent.result, method: uptake, object: remotesettings
  {gCommonEventInfo[127], 7413, 7420},
  // category: uptake.remotecontent.result, method: uptake, object: normandy
  {gCommonEventInfo[127], 7413, 2444},
  // category: urlbar, method: abandonment, object: blur
  {gCommonEventInfo[128], 7435, 7447},
  // category: urlbar, method: engagement, object: click
  {gCommonEventInfo[129], 7452, 5130},
  // category: urlbar, method: engagement, object: enter
  {gCommonEventInfo[129], 7452, 4188},
  // category: urlbar, method: engagement, object: paste_go
  {gCommonEventInfo[129], 7452, 7463},
  // category: urlbar, method: engagement, object: drop_go
  {gCommonEventInfo[129], 7452, 7472},
  // category: webrtc.ui, method: share_display, object: screen
  {gCommonEventInfo[130], 7480, 7494},
  // category: webrtc.ui, method: share_display, object: window
  {gCommonEventInfo[130], 7480, 7501},
  // category: webrtc.ui, method: share_display, object: browser_window
  {gCommonEventInfo[130], 7480, 7508},
  // category: webrtc.ui, method: show_indicator, object: show_indicator
  {gCommonEventInfo[131], 7523, 7523},
  // category: ysod, method: shown, object: ysod
  {gCommonEventInfo[132], 7213, 2763},
  // category: zero_byte_load, method: load, object: ftl
  {gCommonEventInfo[133], 5341, 7538},
  // category: zero_byte_load, method: load, object: dtd
  {gCommonEventInfo[133], 5341, 7542},
  // category: zero_byte_load, method: load, object: properties
  {gCommonEventInfo[133], 5341, 7546},
  // category: zero_byte_load, method: load, object: js
  {gCommonEventInfo[133], 5341, 7557},
  // category: zero_byte_load, method: load, object: xml
  {gCommonEventInfo[133], 5341, 7560},
  // category: zero_byte_load, method: load, object: xhtml
  {gCommonEventInfo[133], 5341, 7564},
  // category: zero_byte_load, method: load, object: css
  {gCommonEventInfo[133], 5341, 7570},
  // category: zero_byte_load, method: load, object: json
  {gCommonEventInfo[133], 5341, 7574},
  // category: zero_byte_load, method: load, object: html
  {gCommonEventInfo[133], 5341, 7579},
  // category: zero_byte_load, method: load, object: png
  {gCommonEventInfo[133], 5341, 7584},
  // category: zero_byte_load, method: load, object: svg
  {gCommonEventInfo[133], 5341, 7588},
  // category: zero_byte_load, method: load, object: others
  {gCommonEventInfo[133], 5341, 7592},
};
static_assert(sizeof(gEventInfo) <= UINT32_MAX, "index overflow");

#if defined(_MSC_VER) && !defined(__clang__)
const char gEventsStringTable[] = {
#else
constexpr char gEventsStringTable[] = {
#endif
  /*     0 - "addon_version" */ 'a', 'd', 'd', 'o', 'n', '_', 'v', 'e', 'r', 's', 'i', 'o', 'n', '\0',
  /*    14 - "page" */ 'p', 'a', 'g', 'e', '\0',
  /*    19 - "session_id" */ 's', 'e', 's', 's', 'i', 'o', 'n', '_', 'i', 'd', '\0',
  /*    30 - "user_prefs" */ 'u', 's', 'e', 'r', '_', 'p', 'r', 'e', 'f', 's', '\0',
  /*    41 - "action_position" */ 'a', 'c', 't', 'i', 'o', 'n', '_', 'p', 'o', 's', 'i', 't', 'i', 'o', 'n', '\0',
  /*    57 - "action" */ 'a', 'c', 't', 'i', 'o', 'n', '\0',
  /*    64 - "addonId" */ 'a', 'd', 'd', 'o', 'n', 'I', 'd', '\0',
  /*    72 - "type" */ 't', 'y', 'p', 'e', '\0',
  /*    77 - "view" */ 'v', 'i', 'e', 'w', '\0',
  /*    82 - "addon_id" */ 'a', 'd', 'd', 'o', 'n', '_', 'i', 'd', '\0',
  /*    91 - "download_time" */ 'd', 'o', 'w', 'n', 'l', 'o', 'a', 'd', '_', 't', 'i', 'm', 'e', '\0',
  /*   105 - "error" */ 'e', 'r', 'r', 'o', 'r', '\0',
  /*   111 - "method" */ 'm', 'e', 't', 'h', 'o', 'd', '\0',
  /*   118 - "num_strings" */ 'n', 'u', 'm', '_', 's', 't', 'r', 'i', 'n', 'g', 's', '\0',
  /*   130 - "source" */ 's', 'o', 'u', 'r', 'c', 'e', '\0',
  /*   137 - "step" */ 's', 't', 'e', 'p', '\0',
  /*   142 - "updated_from" */ 'u', 'p', 'd', 'a', 't', 'e', 'd', '_', 'f', 'r', 'o', 'm', '\0',
  /*   155 - "utm_campaign" */ 'u', 't', 'm', '_', 'c', 'a', 'm', 'p', 'a', 'i', 'g', 'n', '\0',
  /*   168 - "utm_content" */ 'u', 't', 'm', '_', 'c', 'o', 'n', 't', 'e', 'n', 't', '\0',
  /*   180 - "utm_medium" */ 'u', 't', 'm', '_', 'm', 'e', 'd', 'i', 'u', 'm', '\0',
  /*   191 - "utm_source" */ 'u', 't', 'm', '_', 's', 'o', 'u', 'r', 'c', 'e', '\0',
  /*   202 - "addon_type" */ 'a', 'd', 'd', 'o', 'n', '_', 't', 'y', 'p', 'e', '\0',
  /*   213 - "error_type" */ 'e', 'r', 'r', 'o', 'r', '_', 't', 'y', 'p', 'e', '\0',
  /*   224 - "cc_exp" */ 'c', 'c', '_', 'e', 'x', 'p', '\0',
  /*   231 - "cc_exp_found" */ 'c', 'c', '_', 'e', 'x', 'p', '_', 'f', 'o', 'u', 'n', 'd', '\0',
  /*   244 - "cc_name" */ 'c', 'c', '_', 'n', 'a', 'm', 'e', '\0',
  /*   252 - "cc_name_found" */ 'c', 'c', '_', 'n', 'a', 'm', 'e', '_', 'f', 'o', 'u', 'n', 'd', '\0',
  /*   266 - "cc_number" */ 'c', 'c', '_', 'n', 'u', 'm', 'b', 'e', 'r', '\0',
  /*   276 - "cc_number_found" */ 'c', 'c', '_', 'n', 'u', 'm', 'b', 'e', 'r', '_', 'f', 'o', 'u', 'n', 'd', '\0',
  /*   292 - "field_name" */ 'f', 'i', 'e', 'l', 'd', '_', 'n', 'a', 'm', 'e', '\0',
  /*   303 - "fields_auto" */ 'f', 'i', 'e', 'l', 'd', 's', '_', 'a', 'u', 't', 'o', '\0',
  /*   315 - "fields_modified" */ 'f', 'i', 'e', 'l', 'd', 's', '_', 'm', 'o', 'd', 'i', 'f', 'i', 'e', 'd', '\0',
  /*   331 - "fields_not_auto" */ 'f', 'i', 'e', 'l', 'd', 's', '_', 'n', 'o', 't', '_', 'a', 'u', 't', 'o', '\0',
  /*   347 - "host" */ 'h', 'o', 's', 't', '\0',
  /*   352 - "width" */ 'w', 'i', 'd', 't', 'h', '\0',
  /*   358 - "num_tabs" */ 'n', 'u', 'm', '_', 't', 'a', 'b', 's', '\0',
  /*   367 - "reason" */ 'r', 'e', 'a', 's', 'o', 'n', '\0',
  /*   374 - "connection_id" */ 'c', 'o', 'n', 'n', 'e', 'c', 't', 'i', 'o', 'n', '_', 'i', 'd', '\0',
  /*   388 - "connection_type" */ 'c', 'o', 'n', 'n', 'e', 'c', 't', 'i', 'o', 'n', '_', 't', 'y', 'p', 'e', '\0',
  /*   404 - "runtime_id" */ 'r', 'u', 'n', 't', 'i', 'm', 'e', '_', 'i', 'd', '\0',
  /*   415 - "status" */ 's', 't', 'a', 't', 'u', 's', '\0',
  /*   422 - "device_name" */ 'd', 'e', 'v', 'i', 'c', 'e', '_', 'n', 'a', 'm', 'e', '\0',
  /*   434 - "made_changes" */ 'm', 'a', 'd', 'e', '_', 'c', 'h', 'a', 'n', 'g', 'e', 's', '\0',
  /*   447 - "time_open" */ 't', 'i', 'm', 'e', '_', 'o', 'p', 'e', 'n', '\0',
  /*   457 - "cold" */ 'c', 'o', 'l', 'd', '\0',
  /*   462 - "message_count" */ 'm', 'e', 's', 's', 'a', 'g', 'e', '_', 'c', 'o', 'u', 'n', 't', '\0',
  /*   476 - "panel_name" */ 'p', 'a', 'n', 'e', 'l', '_', 'n', 'a', 'm', 'e', '\0',
  /*   487 - "start_state" */ 's', 't', 'a', 'r', 't', '_', 's', 't', 'a', 't', 'e', '\0',
  /*   499 - "input" */ 'i', 'n', 'p', 'u', 't', '\0',
  /*   505 - "lines" */ 'l', 'i', 'n', 'e', 's', '\0',
  /*   511 - "next_panel" */ 'n', 'e', 'x', 't', '_', 'p', 'a', 'n', 'e', 'l', '\0',
  /*   522 - "active" */ 'a', 'c', 't', 'i', 'v', 'e', '\0',
  /*   529 - "inactive" */ 'i', 'n', 'a', 'c', 't', 'i', 'v', 'e', '\0',
  /*   538 - "trigger" */ 't', 'r', 'i', 'g', 'g', 'e', 'r', '\0',
  /*   546 - "runtime_type" */ 'r', 'u', 'n', 't', 'i', 'm', 'e', '_', 't', 'y', 'p', 'e', '\0',
  /*   559 - "target_type" */ 't', 'a', 'r', 'g', 'e', 't', '_', 't', 'y', 'p', 'e', '\0',
  /*   571 - "entrypoint" */ 'e', 'n', 't', 'r', 'y', 'p', 'o', 'i', 'n', 't', '\0',
  /*   582 - "first_panel" */ 'f', 'i', 'r', 's', 't', '_', 'p', 'a', 'n', 'e', 'l', '\0',
  /*   594 - "shortcut" */ 's', 'h', 'o', 'r', 't', 'c', 'u', 't', '\0',
  /*   603 - "splitconsole" */ 's', 'p', 'l', 'i', 't', 'c', 'o', 'n', 's', 'o', 'l', 'e', '\0',
  /*   616 - "lib_stacks" */ 'l', 'i', 'b', '_', 's', 't', 'a', 'c', 'k', 's', '\0',
  /*   627 - "caught_exceptio" */ 'c', 'a', 'u', 'g', 'h', 't', '_', 'e', 'x', 'c', 'e', 'p', 't', 'i', 'o', '\0',
  /*   643 - "exceptions" */ 'e', 'x', 'c', 'e', 'p', 't', 'i', 'o', 'n', 's', '\0',
  /*   654 - "functionality" */ 'f', 'u', 'n', 'c', 't', 'i', 'o', 'n', 'a', 'l', 'i', 't', 'y', '\0',
  /*   668 - "runtime_name" */ 'r', 'u', 'n', 't', 'i', 'm', 'e', '_', 'n', 'a', 'm', 'e', '\0',
  /*   681 - "runtime_os" */ 'r', 'u', 'n', 't', 'i', 'm', 'e', '_', 'o', 's', '\0',
  /*   692 - "runtime_version" */ 'r', 'u', 'n', 't', 'i', 'm', 'e', '_', 'v', 'e', 'r', 's', 'i', 'o', 'n', '\0',
  /*   708 - "page_type" */ 'p', 'a', 'g', 'e', '_', 't', 'y', 'p', 'e', '\0',
  /*   718 - "newpanel" */ 'n', 'e', 'w', 'p', 'a', 'n', 'e', 'l', '\0',
  /*   727 - "oldpanel" */ 'o', 'l', 'd', 'p', 'a', 'n', 'e', 'l', '\0',
  /*   736 - "os" */ 'o', 's', '\0',
  /*   739 - "mode" */ 'm', 'o', 'd', 'e', '\0',
  /*   744 - "prompt_enabled" */ 'p', 'r', 'o', 'm', 'p', 't', '_', 'e', 'n', 'a', 'b', 'l', 'e', 'd', '\0',
  /*   759 - "browserParent" */ 'b', 'r', 'o', 'w', 's', 'e', 'r', 'P', 'a', 'r', 'e', 'n', 't', '\0',
  /*   773 - "canary" */ 'c', 'a', 'n', 'a', 'r', 'y', '\0',
  /*   780 - "evaluateReason" */ 'e', 'v', 'a', 'l', 'u', 'a', 't', 'e', 'R', 'e', 'a', 's', 'o', 'n', '\0',
  /*   795 - "google" */ 'g', 'o', 'o', 'g', 'l', 'e', '\0',
  /*   802 - "modifiedRoots" */ 'm', 'o', 'd', 'i', 'f', 'i', 'e', 'd', 'R', 'o', 'o', 't', 's', '\0',
  /*   816 - "policy" */ 'p', 'o', 'l', 'i', 'c', 'y', '\0',
  /*   823 - "steeredProvider" */ 's', 't', 'e', 'e', 'r', 'e', 'd', 'P', 'r', 'o', 'v', 'i', 'd', 'e', 'r', '\0',
  /*   839 - "thirdPartyRoots" */ 't', 'h', 'i', 'r', 'd', 'P', 'a', 'r', 't', 'y', 'R', 'o', 'o', 't', 's', '\0',
  /*   855 - "youtube" */ 'y', 'o', 'u', 't', 'u', 'b', 'e', '\0',
  /*   863 - "zscalerCanary" */ 'z', 's', 'c', 'a', 'l', 'e', 'r', 'C', 'a', 'n', 'a', 'r', 'y', '\0',
  /*   877 - "canaries" */ 'c', 'a', 'n', 'a', 'r', 'i', 'e', 's', '\0',
  /*   886 - "captiveState" */ 'c', 'a', 'p', 't', 'i', 'v', 'e', 'S', 't', 'a', 't', 'e', '\0',
  /*   899 - "enterprise" */ 'e', 'n', 't', 'e', 'r', 'p', 'r', 'i', 's', 'e', '\0',
  /*   910 - "filtering" */ 'f', 'i', 'l', 't', 'e', 'r', 'i', 'n', 'g', '\0',
  /*   920 - "networkID" */ 'n', 'e', 't', 'w', 'o', 'r', 'k', 'I', 'D', '\0',
  /*   930 - "platform" */ 'p', 'l', 'a', 't', 'f', 'o', 'r', 'm', '\0',
  /*   939 - "context" */ 'c', 'o', 'n', 't', 'e', 'x', 't', '\0',
  /*   947 - "module" */ 'm', 'o', 'd', 'u', 'l', 'e', '\0',
  /*   954 - "result" */ 'r', 'e', 's', 'u', 'l', 't', '\0',
  /*   961 - "seq" */ 's', 'e', 'q', '\0',
  /*   965 - "source_file" */ 's', 'o', 'u', 'r', 'c', 'e', '_', 'f', 'i', 'l', 'e', '\0',
  /*   977 - "source_line" */ 's', 'o', 'u', 'r', 'c', 'e', '_', 'l', 'i', 'n', 'e', '\0',
  /*   989 - "backend" */ 'b', 'a', 'c', 'k', 'e', 'n', 'd', '\0',
  /*   997 - "data_migrated" */ 'd', 'a', 't', 'a', '_', 'm', 'i', 'g', 'r', 'a', 't', 'e', 'd', '\0',
  /*  1011 - "error_name" */ 'e', 'r', 'r', 'o', 'r', '_', 'n', 'a', 'm', 'e', '\0',
  /*  1022 - "has_jsonfile" */ 'h', 'a', 's', '_', 'j', 's', 'o', 'n', 'f', 'i', 'l', 'e', '\0',
  /*  1035 - "has_olddata" */ 'h', 'a', 's', '_', 'o', 'l', 'd', 'd', 'a', 't', 'a', '\0',
  /*  1047 - "acFieldName" */ 'a', 'c', 'F', 'i', 'e', 'l', 'd', 'N', 'a', 'm', 'e', '\0',
  /*  1059 - "fieldType" */ 'f', 'i', 'e', 'l', 'd', 'T', 'y', 'p', 'e', '\0',
  /*  1069 - "generatedPasswo" */ 'g', 'e', 'n', 'e', 'r', 'a', 't', 'e', 'd', 'P', 'a', 's', 's', 'w', 'o', '\0',
  /*  1085 - "hadPrevious" */ 'h', 'a', 'd', 'P', 'r', 'e', 'v', 'i', 'o', 'u', 's', '\0',
  /*  1097 - "importableLogin" */ 'i', 'm', 'p', 'o', 'r', 't', 'a', 'b', 'l', 'e', 'L', 'o', 'g', 'i', 'n', '\0',
  /*  1113 - "insecureWarning" */ 'i', 'n', 's', 'e', 'c', 'u', 'r', 'e', 'W', 'a', 'r', 'n', 'i', 'n', 'g', '\0',
  /*  1129 - "login" */ 'l', 'o', 'g', 'i', 'n', '\0',
  /*  1135 - "loginsFooter" */ 'l', 'o', 'g', 'i', 'n', 's', 'F', 'o', 'o', 't', 'e', 'r', '\0',
  /*  1148 - "stringLength" */ 's', 't', 'r', 'i', 'n', 'g', 'L', 'e', 'n', 'g', 't', 'h', '\0',
  /*  1161 - "typeWasPassword" */ 't', 'y', 'p', 'e', 'W', 'a', 's', 'P', 'a', 's', 's', 'w', 'o', 'r', 'd', '\0',
  /*  1177 - "fxa" */ 'f', 'x', 'a', '\0',
  /*  1181 - "sync" */ 's', 'y', 'n', 'c', '\0',
  /*  1186 - "fxa_avatar" */ 'f', 'x', 'a', '_', 'a', 'v', 'a', 't', 'a', 'r', '\0',
  /*  1197 - "fxa_status" */ 'f', 'x', 'a', '_', 's', 't', 'a', 't', 'u', 's', '\0',
  /*  1208 - "webExtensionId" */ 'w', 'e', 'b', 'E', 'x', 't', 'e', 'n', 's', 'i', 'o', 'n', 'I', 'd', '\0',
  /*  1223 - "admin_user" */ 'a', 'd', 'm', 'i', 'n', '_', 'u', 's', 'e', 'r', '\0',
  /*  1234 - "build_id" */ 'b', 'u', 'i', 'l', 'd', '_', 'i', 'd', '\0',
  /*  1243 - "default_path" */ 'd', 'e', 'f', 'a', 'u', 'l', 't', '_', 'p', 'a', 't', 'h', '\0',
  /*  1256 - "from_msi" */ 'f', 'r', 'o', 'm', '_', 'm', 's', 'i', '\0',
  /*  1265 - "install_existed" */ 'i', 'n', 's', 't', 'a', 'l', 'l', '_', 'e', 'x', 'i', 's', 't', 'e', 'd', '\0',
  /*  1281 - "profdir_existed" */ 'p', 'r', 'o', 'f', 'd', 'i', 'r', '_', 'e', 'x', 'i', 's', 't', 'e', 'd', '\0',
  /*  1297 - "silent" */ 's', 'i', 'l', 'e', 'n', 't', '\0',
  /*  1304 - "version" */ 'v', 'e', 'r', 's', 'i', 'o', 'n', '\0',
  /*  1312 - "installId" */ 'i', 'n', 's', 't', 'a', 'l', 'l', 'I', 'd', '\0',
  /*  1322 - "branches" */ 'b', 'r', 'a', 'n', 'c', 'h', 'e', 's', '\0',
  /*  1331 - "engine" */ 'e', 'n', 'g', 'i', 'n', 'e', '\0',
  /*  1338 - "attemptCount" */ 'a', 't', 't', 'e', 'm', 'p', 't', 'C', 'o', 'u', 'n', 't', '\0',
  /*  1351 - "captivePortal" */ 'c', 'a', 'p', 't', 'i', 'v', 'e', 'P', 'o', 'r', 't', 'a', 'l', '\0',
  /*  1365 - "contextReason" */ 'c', 'o', 'n', 't', 'e', 'x', 't', 'R', 'e', 'a', 's', 'o', 'n', '\0',
  /*  1379 - "failedLookups" */ 'f', 'a', 'i', 'l', 'e', 'd', 'L', 'o', 'o', 'k', 'u', 'p', 's', '\0',
  /*  1393 - "results" */ 'r', 'e', 's', 'u', 'l', 't', 's', '\0',
  /*  1401 - "time" */ 't', 'i', 'm', 'e', '\0',
  /*  1406 - "addonVersion" */ 'a', 'd', 'd', 'o', 'n', 'V', 'e', 'r', 's', 'i', 'o', 'n', '\0',
  /*  1419 - "branch" */ 'b', 'r', 'a', 'n', 'c', 'h', '\0',
  /*  1426 - "enrollmentId" */ 'e', 'n', 'r', 'o', 'l', 'l', 'm', 'e', 'n', 't', 'I', 'd', '\0',
  /*  1439 - "experimentType" */ 'e', 'x', 'p', 'e', 'r', 'i', 'm', 'e', 'n', 't', 'T', 'y', 'p', 'e', '\0',
  /*  1454 - "conflictingSlug" */ 'c', 'o', 'n', 'f', 'l', 'i', 'c', 't', 'i', 'n', 'g', 'S', 'l', 'u', 'g', '\0',
  /*  1470 - "detail" */ 'd', 'e', 't', 'a', 'i', 'l', '\0',
  /*  1477 - "preference" */ 'p', 'r', 'e', 'f', 'e', 'r', 'e', 'n', 'c', 'e', '\0',
  /*  1488 - "branchSlug" */ 'b', 'r', 'a', 'n', 'c', 'h', 'S', 'l', 'u', 'g', '\0',
  /*  1499 - "featureId" */ 'f', 'e', 'a', 't', 'u', 'r', 'e', 'I', 'd', '\0',
  /*  1509 - "changedPref" */ 'c', 'h', 'a', 'n', 'g', 'e', 'd', 'P', 'r', 'e', 'f', '\0',
  /*  1521 - "didResetValue" */ 'd', 'i', 'd', 'R', 'e', 's', 'e', 't', 'V', 'a', 'l', 'u', 'e', '\0',
  /*  1535 - "caller" */ 'c', 'a', 'l', 'l', 'e', 'r', '\0',
  /*  1542 - "originalReason" */ 'o', 'r', 'i', 'g', 'i', 'n', 'a', 'l', 'R', 'e', 'a', 's', 'o', 'n', '\0',
  /*  1557 - "previousState" */ 'p', 'r', 'e', 'v', 'i', 'o', 'u', 's', 'S', 't', 'a', 't', 'e', '\0',
  /*  1571 - "height" */ 'h', 'e', 'i', 'g', 'h', 't', '\0',
  /*  1578 - "screenX" */ 's', 'c', 'r', 'e', 'e', 'n', 'X', '\0',
  /*  1586 - "screenY" */ 's', 'c', 'r', 'e', 'e', 'n', 'Y', '\0',
  /*  1594 - "did_edit_pw" */ 'd', 'i', 'd', '_', 'e', 'd', 'i', 't', '_', 'p', 'w', '\0',
  /*  1606 - "did_edit_un" */ 'd', 'i', 'd', '_', 'e', 'd', 'i', 't', '_', 'u', 'n', '\0',
  /*  1618 - "did_select_pw" */ 'd', 'i', 'd', '_', 's', 'e', 'l', 'e', 'c', 't', '_', 'p', 'w', '\0',
  /*  1632 - "did_select_un" */ 'd', 'i', 'd', '_', 's', 'e', 'l', 'e', 'c', 't', '_', 'u', 'n', '\0',
  /*  1646 - "breached" */ 'b', 'r', 'e', 'a', 'c', 'h', 'e', 'd', '\0',
  /*  1655 - "sort_key" */ 's', 'o', 'r', 't', '_', 'k', 'e', 'y', '\0',
  /*  1664 - "vulnerable" */ 'v', 'u', 'l', 'n', 'e', 'r', 'a', 'b', 'l', 'e', '\0',
  /*  1675 - "auto_admin" */ 'a', 'u', 't', 'o', '_', 'a', 'd', 'm', 'i', 'n', '\0',
  /*  1686 - "require_signon" */ 'r', 'e', 'q', 'u', 'i', 'r', 'e', '_', 's', 'i', 'g', 'n', 'o', 'n', '\0',
  /*  1701 - "filled" */ 'f', 'i', 'l', 'l', 'e', 'd', '\0',
  /*  1708 - "subcategory" */ 's', 'u', 'b', 'c', 'a', 't', 'e', 'g', 'o', 'r', 'y', '\0',
  /*  1720 - "fileinfo" */ 'f', 'i', 'l', 'e', 'i', 'n', 'f', 'o', '\0',
  /*  1729 - "principalType" */ 'p', 'r', 'i', 'n', 'c', 'i', 'p', 'a', 'l', 'T', 'y', 'p', 'e', '\0',
  /*  1743 - "scheme" */ 's', 'c', 'h', 'e', 'm', 'e', '\0',
  /*  1750 - "contenttype" */ 'c', 'o', 'n', 't', 'e', 'n', 't', 't', 'y', 'p', 'e', '\0',
  /*  1762 - "filedetails" */ 'f', 'i', 'l', 'e', 'd', 'e', 't', 'a', 'i', 'l', 's', '\0',
  /*  1774 - "remotetype" */ 'r', 'e', 'm', 'o', 't', 'e', 't', 'y', 'p', 'e', '\0',
  /*  1785 - "domain" */ 'd', 'o', 'm', 'a', 'i', 'n', '\0',
  /*  1792 - "networkUnstable" */ 'n', 'e', 't', 'w', 'o', 'r', 'k', 'U', 'n', 's', 't', 'a', 'b', 'l', 'e', '\0',
  /*  1808 - "retryCount" */ 'r', 'e', 't', 'r', 'y', 'C', 'o', 'u', 'n', 't', '\0',
  /*  1819 - "trr" */ 't', 'r', 'r', '\0',
  /*  1823 - "has_sts" */ 'h', 'a', 's', '_', 's', 't', 's', '\0',
  /*  1831 - "is_frame" */ 'i', 's', '_', 'f', 'r', 'a', 'm', 'e', '\0',
  /*  1840 - "panel_open" */ 'p', 'a', 'n', 'e', 'l', '_', 'o', 'p', 'e', 'n', '\0',
  /*  1851 - "category" */ 'c', 'a', 't', 'e', 'g', 'o', 'r', 'y', '\0',
  /*  1860 - "end_reason" */ 'e', 'n', 'd', '_', 'r', 'e', 'a', 's', 'o', 'n', '\0',
  /*  1871 - "hang_duration" */ 'h', 'a', 'n', 'g', '_', 'd', 'u', 'r', 'a', 't', 'i', 'o', 'n', '\0',
  /*  1885 - "n_tab_deselect" */ 'n', '_', 't', 'a', 'b', '_', 'd', 'e', 's', 'e', 'l', 'e', 'c', 't', '\0',
  /*  1900 - "uptime" */ 'u', 'p', 't', 'i', 'm', 'e', '\0',
  /*  1907 - "uri_type" */ 'u', 'r', 'i', '_', 't', 'y', 'p', 'e', '\0',
  /*  1916 - "wait_count" */ 'w', 'a', 'i', 't', '_', 'c', 'o', 'u', 'n', 't', '\0',
  /*  1927 - "bar" */ 'b', 'a', 'r', '\0',
  /*  1931 - "foo" */ 'f', 'o', 'o', '\0',
  /*  1935 - "key1" */ 'k', 'e', 'y', '1', '\0',
  /*  1940 - "key2" */ 'k', 'e', 'y', '2', '\0',
  /*  1945 - "age" */ 'a', 'g', 'e', '\0',
  /*  1949 - "duration" */ 'd', 'u', 'r', 'a', 't', 'i', 'o', 'n', '\0',
  /*  1958 - "errorName" */ 'e', 'r', 'r', 'o', 'r', 'N', 'a', 'm', 'e', '\0',
  /*  1968 - "timestamp" */ 't', 'i', 'm', 'e', 's', 't', 'a', 'm', 'p', '\0',
  /*  1978 - "elapsed" */ 'e', 'l', 'a', 'p', 's', 'e', 'd', '\0',
  /*  1986 - "numChars" */ 'n', 'u', 'm', 'C', 'h', 'a', 'r', 's', '\0',
  /*  1995 - "numWords" */ 'n', 'u', 'm', 'W', 'o', 'r', 'd', 's', '\0',
  /*  2004 - "provider" */ 'p', 'r', 'o', 'v', 'i', 'd', 'e', 'r', '\0',
  /*  2013 - "selIndex" */ 's', 'e', 'l', 'I', 'n', 'd', 'e', 'x', '\0',
  /*  2022 - "selType" */ 's', 'e', 'l', 'T', 'y', 'p', 'e', '\0',
  /*  2030 - "silence_notifs" */ 's', 'i', 'l', 'e', 'n', 'c', 'e', '_', 'n', 'o', 't', 'i', 'f', 's', '\0',
  /*  2045 - "error_code" */ 'e', 'r', 'r', 'o', 'r', '_', 'c', 'o', 'd', 'e', '\0',
  /*  2056 - "last_line" */ 'l', 'a', 's', 't', '_', 'l', 'i', 'n', 'e', '\0',
  /*  2066 - "last_line_len" */ 'l', 'a', 's', 't', '_', 'l', 'i', 'n', 'e', '_', 'l', 'e', 'n', '\0',
  /*  2080 - "location" */ 'l', 'o', 'c', 'a', 't', 'i', 'o', 'n', '\0',
  /*  2089 - "cancelled" */ 'c', 'a', 'n', 'c', 'e', 'l', 'l', 'e', 'd', '\0',
  /*  2099 - "file_name" */ 'f', 'i', 'l', 'e', '_', 'n', 'a', 'm', 'e', '\0',
  /*  2109 - "activity_stream" */ 'a', 'c', 't', 'i', 'v', 'i', 't', 'y', '_', 's', 't', 'r', 'e', 'a', 'm', '\0',
  /*  2125 - "never" */ 'n', 'e', 'v', 'e', 'r', '\0',
  /*  2131 - "addonsManager" */ 'a', 'd', 'd', 'o', 'n', 's', 'M', 'a', 'n', 'a', 'g', 'e', 'r', '\0',
  /*  2145 - "92.0a1" */ '9', '2', '.', '0', 'a', '1', '\0',
  /*  2152 - "avif" */ 'a', 'v', 'i', 'f', '\0',
  /*  2157 - "contextservices.quicksuggest" */ 'c', 'o', 'n', 't', 'e', 'x', 't', 's', 'e', 'r', 'v', 'i', 'c', 'e', 's', '.', 'q', 'u', 'i', 'c', 'k', 's', 'u', 'g', 'g', 'e', 's', 't', '\0',
  /*  2186 - "creditcard" */ 'c', 'r', 'e', 'd', 'i', 't', 'c', 'a', 'r', 'd', '\0',
  /*  2197 - "93.0a1" */ '9', '3', '.', '0', 'a', '1', '\0',
  /*  2204 - "devtools.main" */ 'd', 'e', 'v', 't', 'o', 'o', 'l', 's', '.', 'm', 'a', 'i', 'n', '\0',
  /*  2218 - "82.0a1" */ '8', '2', '.', '0', 'a', '1', '\0',
  /*  2225 - "80.0a1" */ '8', '0', '.', '0', 'a', '1', '\0',
  /*  2232 - "doh" */ 'd', 'o', 'h', '\0',
  /*  2236 - "dom.quota.try" */ 'd', 'o', 'm', '.', 'q', 'u', 'o', 't', 'a', '.', 't', 'r', 'y', '\0',
  /*  2250 - "downloads" */ 'd', 'o', 'w', 'n', 'l', 'o', 'a', 'd', 's', '\0',
  /*  2260 - "extensions.data" */ 'e', 'x', 't', 'e', 'n', 's', 'i', 'o', 'n', 's', '.', 'd', 'a', 't', 'a', '\0',
  /*  2276 - "95.0a1" */ '9', '5', '.', '0', 'a', '1', '\0',
  /*  2283 - "form_autocomplete" */ 'f', 'o', 'r', 'm', '_', 'a', 'u', 't', 'o', 'c', 'o', 'm', 'p', 'l', 'e', 't', 'e', '\0',
  /*  2301 - "fxa_app_menu" */ 'f', 'x', 'a', '_', 'a', 'p', 'p', '_', 'm', 'e', 'n', 'u', '\0',
  /*  2314 - "fxa_avatar_menu" */ 'f', 'x', 'a', '_', 'a', 'v', 'a', 't', 'a', 'r', '_', 'm', 'e', 'n', 'u', '\0',
  /*  2330 - "homepage" */ 'h', 'o', 'm', 'e', 'p', 'a', 'g', 'e', '\0',
  /*  2339 - "installation" */ 'i', 'n', 's', 't', 'a', 'l', 'l', 'a', 't', 'i', 'o', 'n', '\0',
  /*  2352 - "94.0a1" */ '9', '4', '.', '0', 'a', '1', '\0',
  /*  2359 - "intl.ui.browserLanguage" */ 'i', 'n', 't', 'l', '.', 'u', 'i', '.', 'b', 'r', 'o', 'w', 's', 'e', 'r', 'L', 'a', 'n', 'g', 'u', 'a', 'g', 'e', '\0',
  /*  2383 - "97.0a1" */ '9', '7', '.', '0', 'a', '1', '\0',
  /*  2390 - "jsonfile" */ 'j', 's', 'o', 'n', 'f', 'i', 'l', 'e', '\0',
  /*  2399 - "messaging_experiments" */ 'm', 'e', 's', 's', 'a', 'g', 'i', 'n', 'g', '_', 'e', 'x', 'p', 'e', 'r', 'i', 'm', 'e', 'n', 't', 's', '\0',
  /*  2421 - "navigation" */ 'n', 'a', 'v', 'i', 'g', 'a', 't', 'i', 'o', 'n', '\0',
  /*  2432 - "network.dns" */ 'n', 'e', 't', 'w', 'o', 'r', 'k', '.', 'd', 'n', 's', '\0',
  /*  2444 - "normandy" */ 'n', 'o', 'r', 'm', 'a', 'n', 'd', 'y', '\0',
  /*  2453 - "partner_link" */ 'p', 'a', 'r', 't', 'n', 'e', 'r', '_', 'l', 'i', 'n', 'k', '\0',
  /*  2466 - "pictureinpicture" */ 'p', 'i', 'c', 't', 'u', 'r', 'e', 'i', 'n', 'p', 'i', 'c', 't', 'u', 'r', 'e', '\0',
  /*  2483 - "pwmgr" */ 'p', 'w', 'm', 'g', 'r', '\0',
  /*  2489 - "readermode" */ 'r', 'e', 'a', 'd', 'e', 'r', 'm', 'o', 'd', 'e', '\0',
  /*  2500 - "security" */ 's', 'e', 'c', 'u', 'r', 'i', 't', 'y', '\0',
  /*  2509 - "security.doh.trrPerformance" */ 's', 'e', 'c', 'u', 'r', 'i', 't', 'y', '.', 'd', 'o', 'h', '.', 't', 'r', 'r', 'P', 'e', 'r', 'f', 'o', 'r', 'm', 'a', 'n', 'c', 'e', '\0',
  /*  2537 - "security.ui.app_menu" */ 's', 'e', 'c', 'u', 'r', 'i', 't', 'y', '.', 'u', 'i', '.', 'a', 'p', 'p', '_', 'm', 'e', 'n', 'u', '\0',
  /*  2558 - "security.ui.certerror" */ 's', 'e', 'c', 'u', 'r', 'i', 't', 'y', '.', 'u', 'i', '.', 'c', 'e', 'r', 't', 'e', 'r', 'r', 'o', 'r', '\0',
  /*  2580 - "security.ui.protections" */ 's', 'e', 'c', 'u', 'r', 'i', 't', 'y', '.', 'u', 'i', '.', 'p', 'r', 'o', 't', 'e', 'c', 't', 'i', 'o', 'n', 's', '\0',
  /*  2604 - "security.ui.protectionspopup" */ 's', 'e', 'c', 'u', 'r', 'i', 't', 'y', '.', 'u', 'i', '.', 'p', 'r', 'o', 't', 'e', 'c', 't', 'i', 'o', 'n', 's', 'p', 'o', 'p', 'u', 'p', '\0',
  /*  2633 - "slow_script_warning" */ 's', 'l', 'o', 'w', '_', 's', 'c', 'r', 'i', 'p', 't', '_', 'w', 'a', 'r', 'n', 'i', 'n', 'g', '\0',
  /*  2653 - "telemetry.test" */ 't', 'e', 'l', 'e', 'm', 'e', 't', 'r', 'y', '.', 't', 'e', 's', 't', '\0',
  /*  2668 - "3.0a1" */ '3', '.', '0', 'a', '1', '\0',
  /*  2674 - "999.0a1" */ '9', '9', '9', '.', '0', 'a', '1', '\0',
  /*  2682 - "telemetry.test.second" */ 't', 'e', 'l', 'e', 'm', 'e', 't', 'r', 'y', '.', 't', 'e', 's', 't', '.', 's', 'e', 'c', 'o', 'n', 'd', '\0',
  /*  2704 - "uptake.remotecontent.result" */ 'u', 'p', 't', 'a', 'k', 'e', '.', 'r', 'e', 'm', 'o', 't', 'e', 'c', 'o', 'n', 't', 'e', 'n', 't', '.', 'r', 'e', 's', 'u', 'l', 't', '\0',
  /*  2732 - "urlbar" */ 'u', 'r', 'l', 'b', 'a', 'r', '\0',
  /*  2739 - "webrtc.ui" */ 'w', 'e', 'b', 'r', 't', 'c', '.', 'u', 'i', '\0',
  /*  2749 - "88.0a1" */ '8', '8', '.', '0', 'a', '1', '\0',
  /*  2756 - "87.0a1" */ '8', '7', '.', '0', 'a', '1', '\0',
  /*  2763 - "ysod" */ 'y', 's', 'o', 'd', '\0',
  /*  2768 - "zero_byte_load" */ 'z', 'e', 'r', 'o', '_', 'b', 'y', 't', 'e', '_', 'l', 'o', 'a', 'd', '\0',
  /*  2783 - "end" */ 'e', 'n', 'd', '\0',
  /*  2787 - "session" */ 's', 'e', 's', 's', 'i', 'o', 'n', '\0',
  /*  2795 - "event" */ 'e', 'v', 'e', 'n', 't', '\0',
  /*  2801 - "ARCHIVE_FROM_POCKET" */ 'A', 'R', 'C', 'H', 'I', 'V', 'E', '_', 'F', 'R', 'O', 'M', '_', 'P', 'O', 'C', 'K', 'E', 'T', '\0',
  /*  2821 - "BLOCK" */ 'B', 'L', 'O', 'C', 'K', '\0',
  /*  2827 - "BOOKMARK_ADD" */ 'B', 'O', 'O', 'K', 'M', 'A', 'R', 'K', '_', 'A', 'D', 'D', '\0',
  /*  2840 - "BOOKMARK_DELETE" */ 'B', 'O', 'O', 'K', 'M', 'A', 'R', 'K', '_', 'D', 'E', 'L', 'E', 'T', 'E', '\0',
  /*  2856 - "CLICK" */ 'C', 'L', 'I', 'C', 'K', '\0',
  /*  2862 - "CLICK_PRIVACY_INFO" */ 'C', 'L', 'I', 'C', 'K', '_', 'P', 'R', 'I', 'V', 'A', 'C', 'Y', '_', 'I', 'N', 'F', 'O', '\0',
  /*  2881 - "CLOSE_NEWTAB_PREFS" */ 'C', 'L', 'O', 'S', 'E', '_', 'N', 'E', 'W', 'T', 'A', 'B', '_', 'P', 'R', 'E', 'F', 'S', '\0',
  /*  2900 - "SHOW_PERSONALIZE" */ 'S', 'H', 'O', 'W', '_', 'P', 'E', 'R', 'S', 'O', 'N', 'A', 'L', 'I', 'Z', 'E', '\0',
  /*  2917 - "HIDE_PERSONALIZE" */ 'H', 'I', 'D', 'E', '_', 'P', 'E', 'R', 'S', 'O', 'N', 'A', 'L', 'I', 'Z', 'E', '\0',
  /*  2934 - "DELETE" */ 'D', 'E', 'L', 'E', 'T', 'E', '\0',
  /*  2941 - "DELETE_FROM_POCKET" */ 'D', 'E', 'L', 'E', 'T', 'E', '_', 'F', 'R', 'O', 'M', '_', 'P', 'O', 'C', 'K', 'E', 'T', '\0',
  /*  2960 - "DELETE_CONFIRM" */ 'D', 'E', 'L', 'E', 'T', 'E', '_', 'C', 'O', 'N', 'F', 'I', 'R', 'M', '\0',
  /*  2975 - "DIALOG_CANCEL" */ 'D', 'I', 'A', 'L', 'O', 'G', '_', 'C', 'A', 'N', 'C', 'E', 'L', '\0',
  /*  2989 - "DIALOG_OPEN" */ 'D', 'I', 'A', 'L', 'O', 'G', '_', 'O', 'P', 'E', 'N', '\0',
  /*  3001 - "DRAG" */ 'D', 'R', 'A', 'G', '\0',
  /*  3006 - "DROP" */ 'D', 'R', 'O', 'P', '\0',
  /*  3011 - "MIGRATION_CANCEL" */ 'M', 'I', 'G', 'R', 'A', 'T', 'I', 'O', 'N', '_', 'C', 'A', 'N', 'C', 'E', 'L', '\0',
  /*  3028 - "MIGRATION_START" */ 'M', 'I', 'G', 'R', 'A', 'T', 'I', 'O', 'N', '_', 'S', 'T', 'A', 'R', 'T', '\0',
  /*  3044 - "OPEN_NEWTAB_PREFS" */ 'O', 'P', 'E', 'N', '_', 'N', 'E', 'W', 'T', 'A', 'B', '_', 'P', 'R', 'E', 'F', 'S', '\0',
  /*  3062 - "OPEN_NEW_WINDOW" */ 'O', 'P', 'E', 'N', '_', 'N', 'E', 'W', '_', 'W', 'I', 'N', 'D', 'O', 'W', '\0',
  /*  3078 - "OPEN_PRIVATE_WINDOW" */ 'O', 'P', 'E', 'N', '_', 'P', 'R', 'I', 'V', 'A', 'T', 'E', '_', 'W', 'I', 'N', 'D', 'O', 'W', '\0',
  /*  3098 - "PIN" */ 'P', 'I', 'N', '\0',
  /*  3102 - "PREF_CHANGED" */ 'P', 'R', 'E', 'F', '_', 'C', 'H', 'A', 'N', 'G', 'E', 'D', '\0',
  /*  3115 - "PREVIEW_REQUEST" */ 'P', 'R', 'E', 'V', 'I', 'E', 'W', '_', 'R', 'E', 'Q', 'U', 'E', 'S', 'T', '\0',
  /*  3131 - "SAVE_TO_POCKET" */ 'S', 'A', 'V', 'E', '_', 'T', 'O', '_', 'P', 'O', 'C', 'K', 'E', 'T', '\0',
  /*  3146 - "SEARCH" */ 'S', 'E', 'A', 'R', 'C', 'H', '\0',
  /*  3153 - "SEARCH_EDIT_ADD" */ 'S', 'E', 'A', 'R', 'C', 'H', '_', 'E', 'D', 'I', 'T', '_', 'A', 'D', 'D', '\0',
  /*  3169 - "SEARCH_EDIT_CLOSE" */ 'S', 'E', 'A', 'R', 'C', 'H', '_', 'E', 'D', 'I', 'T', '_', 'C', 'L', 'O', 'S', 'E', '\0',
  /*  3187 - "SEARCH_EDIT_DELETE" */ 'S', 'E', 'A', 'R', 'C', 'H', '_', 'E', 'D', 'I', 'T', '_', 'D', 'E', 'L', 'E', 'T', 'E', '\0',
  /*  3206 - "SEARCH_HANDOFF" */ 'S', 'E', 'A', 'R', 'C', 'H', '_', 'H', 'A', 'N', 'D', 'O', 'F', 'F', '\0',
  /*  3221 - "SHOW_PRIVACY_INFO" */ 'S', 'H', 'O', 'W', '_', 'P', 'R', 'I', 'V', 'A', 'C', 'Y', '_', 'I', 'N', 'F', 'O', '\0',
  /*  3239 - "SKIPPED_SIGNIN" */ 'S', 'K', 'I', 'P', 'P', 'E', 'D', '_', 'S', 'I', 'G', 'N', 'I', 'N', '\0',
  /*  3254 - "SUBMIT_EMAIL" */ 'S', 'U', 'B', 'M', 'I', 'T', '_', 'E', 'M', 'A', 'I', 'L', '\0',
  /*  3267 - "DISCLAIMER_ACKED" */ 'D', 'I', 'S', 'C', 'L', 'A', 'I', 'M', 'E', 'R', '_', 'A', 'C', 'K', 'E', 'D', '\0',
  /*  3284 - "MENU_ADD_SEARCH" */ 'M', 'E', 'N', 'U', '_', 'A', 'D', 'D', '_', 'S', 'E', 'A', 'R', 'C', 'H', '\0',
  /*  3300 - "MENU_ADD_TOPSITE" */ 'M', 'E', 'N', 'U', '_', 'A', 'D', 'D', '_', 'T', 'O', 'P', 'S', 'I', 'T', 'E', '\0',
  /*  3317 - "MENU_COLLAPSE" */ 'M', 'E', 'N', 'U', '_', 'C', 'O', 'L', 'L', 'A', 'P', 'S', 'E', '\0',
  /*  3331 - "MENU_EXPAND" */ 'M', 'E', 'N', 'U', '_', 'E', 'X', 'P', 'A', 'N', 'D', '\0',
  /*  3343 - "MENU_MANAGE" */ 'M', 'E', 'N', 'U', '_', 'M', 'A', 'N', 'A', 'G', 'E', '\0',
  /*  3355 - "MENU_MOVE_DOWN" */ 'M', 'E', 'N', 'U', '_', 'M', 'O', 'V', 'E', '_', 'D', 'O', 'W', 'N', '\0',
  /*  3370 - "MENU_MOVE_UP" */ 'M', 'E', 'N', 'U', '_', 'M', 'O', 'V', 'E', '_', 'U', 'P', '\0',
  /*  3383 - "MENU_PRIVACY_NOTICE" */ 'M', 'E', 'N', 'U', '_', 'P', 'R', 'I', 'V', 'A', 'C', 'Y', '_', 'N', 'O', 'T', 'I', 'C', 'E', '\0',
  /*  3403 - "MENU_REMOVE" */ 'M', 'E', 'N', 'U', '_', 'R', 'E', 'M', 'O', 'V', 'E', '\0',
  /*  3415 - "TOP_SITES_EDIT" */ 'T', 'O', 'P', '_', 'S', 'I', 'T', 'E', 'S', '_', 'E', 'D', 'I', 'T', '\0',
  /*  3430 - "TOP_SITES_EDIT_CLOSE" */ 'T', 'O', 'P', '_', 'S', 'I', 'T', 'E', 'S', '_', 'E', 'D', 'I', 'T', '_', 'C', 'L', 'O', 'S', 'E', '\0',
  /*  3451 - "TOPSITE_SPONSOR_INFO" */ 'T', 'O', 'P', 'S', 'I', 'T', 'E', '_', 'S', 'P', 'O', 'N', 'S', 'O', 'R', '_', 'I', 'N', 'F', 'O', '\0',
  /*  3472 - "UNPIN" */ 'U', 'N', 'P', 'I', 'N', '\0',
  /*  3478 - "aboutAddons" */ 'a', 'b', 'o', 'u', 't', 'A', 'd', 'd', 'o', 'n', 's', '\0',
  /*  3490 - "browserAction" */ 'b', 'r', 'o', 'w', 's', 'e', 'r', 'A', 'c', 't', 'i', 'o', 'n', '\0',
  /*  3504 - "customize" */ 'c', 'u', 's', 't', 'o', 'm', 'i', 'z', 'e', '\0',
  /*  3514 - "pageAction" */ 'p', 'a', 'g', 'e', 'A', 'c', 't', 'i', 'o', 'n', '\0',
  /*  3525 - "doorhanger" */ 'd', 'o', 'o', 'r', 'h', 'a', 'n', 'g', 'e', 'r', '\0',
  /*  3536 - "appUpgrade" */ 'a', 'p', 'p', 'U', 'p', 'g', 'r', 'a', 'd', 'e', '\0',
  /*  3547 - "install" */ 'i', 'n', 's', 't', 'a', 'l', 'l', '\0',
  /*  3555 - "extension" */ 'e', 'x', 't', 'e', 'n', 's', 'i', 'o', 'n', '\0',
  /*  3565 - "theme" */ 't', 'h', 'e', 'm', 'e', '\0',
  /*  3571 - "locale" */ 'l', 'o', 'c', 'a', 'l', 'e', '\0',
  /*  3578 - "dictionary" */ 'd', 'i', 'c', 't', 'i', 'o', 'n', 'a', 'r', 'y', '\0',
  /*  3589 - "other" */ 'o', 't', 'h', 'e', 'r', '\0',
  /*  3595 - "unknown" */ 'u', 'n', 'k', 'n', 'o', 'w', 'n', '\0',
  /*  3603 - "update" */ 'u', 'p', 'd', 'a', 't', 'e', '\0',
  /*  3610 - "install_stats" */ 'i', 'n', 's', 't', 'a', 'l', 'l', '_', 's', 't', 'a', 't', 's', '\0',
  /*  3624 - "link" */ 'l', 'i', 'n', 'k', '\0',
  /*  3629 - "aboutPreferences" */ 'a', 'b', 'o', 'u', 't', 'P', 'r', 'e', 'f', 'e', 'r', 'e', 'n', 'c', 'e', 's', '\0',
  /*  3646 - "disable" */ 'd', 'i', 's', 'a', 'b', 'l', 'e', '\0',
  /*  3654 - "enable" */ 'e', 'n', 'a', 'b', 'l', 'e', '\0',
  /*  3661 - "sideload_prompt" */ 's', 'i', 'd', 'e', 'l', 'o', 'a', 'd', '_', 'p', 'r', 'o', 'm', 'p', 't', '\0',
  /*  3677 - "uninstall" */ 'u', 'n', 'i', 'n', 's', 't', 'a', 'l', 'l', '\0',
  /*  3687 - "report" */ 'r', 'e', 'p', 'o', 'r', 't', '\0',
  /*  3694 - "amo" */ 'a', 'm', 'o', '\0',
  /*  3698 - "menu" */ 'm', 'e', 'n', 'u', '\0',
  /*  3703 - "toolbar_context_menu" */ 't', 'o', 'o', 'l', 'b', 'a', 'r', '_', 'c', 'o', 'n', 't', 'e', 'x', 't', '_', 'm', 'e', 'n', 'u', '\0',
  /*  3724 - "dav1d_get_picture" */ 'd', 'a', 'v', '1', 'd', '_', 'g', 'e', 't', '_', 'p', 'i', 'c', 't', 'u', 'r', 'e', '\0',
  /*  3742 - "return_value" */ 'r', 'e', 't', 'u', 'r', 'n', '_', 'v', 'a', 'l', 'u', 'e', '\0',
  /*  3755 - "enable_toggled" */ 'e', 'n', 'a', 'b', 'l', 'e', '_', 't', 'o', 'g', 'g', 'l', 'e', 'd', '\0',
  /*  3770 - "enabled" */ 'e', 'n', 'a', 'b', 'l', 'e', 'd', '\0',
  /*  3778 - "disabled" */ 'd', 'i', 's', 'a', 'b', 'l', 'e', 'd', '\0',
  /*  3787 - "detected" */ 'd', 'e', 't', 'e', 'c', 't', 'e', 'd', '\0',
  /*  3796 - "cc_form" */ 'c', 'c', '_', 'f', 'o', 'r', 'm', '\0',
  /*  3804 - "popup_shown" */ 'p', 'o', 'p', 'u', 'p', '_', 's', 'h', 'o', 'w', 'n', '\0',
  /*  3816 - "filled_modified" */ 'f', 'i', 'l', 'l', 'e', 'd', '_', 'm', 'o', 'd', 'i', 'f', 'i', 'e', 'd', '\0',
  /*  3832 - "submitted" */ 's', 'u', 'b', 'm', 'i', 't', 't', 'e', 'd', '\0',
  /*  3842 - "show" */ 's', 'h', 'o', 'w', '\0',
  /*  3847 - "capture_doorhanger" */ 'c', 'a', 'p', 't', 'u', 'r', 'e', '_', 'd', 'o', 'o', 'r', 'h', 'a', 'n', 'g', 'e', 'r', '\0',
  /*  3866 - "update_doorhanger" */ 'u', 'p', 'd', 'a', 't', 'e', '_', 'd', 'o', 'o', 'r', 'h', 'a', 'n', 'g', 'e', 'r', '\0',
  /*  3884 - "save" */ 's', 'a', 'v', 'e', '\0',
  /*  3889 - "cancel" */ 'c', 'a', 'n', 'c', 'e', 'l', '\0',
  /*  3896 - "manage" */ 'm', 'a', 'n', 'a', 'g', 'e', '\0',
  /*  3903 - "add" */ 'a', 'd', 'd', '\0',
  /*  3907 - "delete" */ 'd', 'e', 'l', 'e', 't', 'e', '\0',
  /*  3914 - "show_entry" */ 's', 'h', 'o', 'w', '_', 'e', 'n', 't', 'r', 'y', '\0',
  /*  3925 - "edit" */ 'e', 'd', 'i', 't', '\0',
  /*  3930 - "activate" */ 'a', 'c', 't', 'i', 'v', 'a', 't', 'e', '\0',
  /*  3939 - "responsive_design" */ 'r', 'e', 's', 'p', 'o', 'n', 's', 'i', 'v', 'e', '_', 'd', 'e', 's', 'i', 'g', 'n', '\0',
  /*  3957 - "split_console" */ 's', 'p', 'l', 'i', 't', '_', 'c', 'o', 'n', 's', 'o', 'l', 'e', '\0',
  /*  3971 - "add_breakpoint" */ 'a', 'd', 'd', '_', 'b', 'r', 'e', 'a', 'k', 'p', 'o', 'i', 'n', 't', '\0',
  /*  3986 - "debugger" */ 'd', 'e', 'b', 'u', 'g', 'g', 'e', 'r', '\0',
  /*  3995 - "blackbox" */ 'b', 'l', 'a', 'c', 'k', 'b', 'o', 'x', '\0',
  /*  4004 - "close" */ 'c', 'l', 'o', 's', 'e', '\0',
  /*  4010 - "tools" */ 't', 'o', 'o', 'l', 's', '\0',
  /*  4016 - "close_adbg" */ 'c', 'l', 'o', 's', 'e', '_', 'a', 'd', 'b', 'g', '\0',
  /*  4027 - "aboutdebugging" */ 'a', 'b', 'o', 'u', 't', 'd', 'e', 'b', 'u', 'g', 'g', 'i', 'n', 'g', '\0',
  /*  4042 - "close_source_tab" */ 'c', 'l', 'o', 's', 'e', '_', 's', 'o', 'u', 'r', 'c', 'e', '_', 't', 'a', 'b', '\0',
  /*  4059 - "connection_attempt" */ 'c', 'o', 'n', 'n', 'e', 'c', 't', 'i', 'o', 'n', '_', 'a', 't', 't', 'e', 'm', 'p', 't', '\0',
  /*  4078 - "continue" */ 'c', 'o', 'n', 't', 'i', 'n', 'u', 'e', '\0',
  /*  4087 - "deactivate" */ 'd', 'e', 'a', 'c', 't', 'i', 'v', 'a', 't', 'e', '\0',
  /*  4098 - "device_added" */ 'd', 'e', 'v', 'i', 'c', 'e', '_', 'a', 'd', 'd', 'e', 'd', '\0',
  /*  4111 - "device_removed" */ 'd', 'e', 'v', 'i', 'c', 'e', '_', 'r', 'e', 'm', 'o', 'v', 'e', 'd', '\0',
  /*  4126 - "edit_html" */ 'e', 'd', 'i', 't', '_', 'h', 't', 'm', 'l', '\0',
  /*  4136 - "inspector" */ 'i', 'n', 's', 'p', 'e', 'c', 't', 'o', 'r', '\0',
  /*  4146 - "edit_resend" */ 'e', 'd', 'i', 't', '_', 'r', 'e', 's', 'e', 'n', 'd', '\0',
  /*  4158 - "netmonitor" */ 'n', 'e', 't', 'm', 'o', 'n', 'i', 't', 'o', 'r', '\0',
  /*  4169 - "edit_rule" */ 'e', 'd', 'i', 't', '_', 'r', 'u', 'l', 'e', '\0',
  /*  4179 - "ruleview" */ 'r', 'u', 'l', 'e', 'v', 'i', 'e', 'w', '\0',
  /*  4188 - "enter" */ 'e', 'n', 't', 'e', 'r', '\0',
  /*  4194 - "accessibility" */ 'a', 'c', 'c', 'e', 's', 's', 'i', 'b', 'i', 'l', 'i', 't', 'y', '\0',
  /*  4208 - "application" */ 'a', 'p', 'p', 'l', 'i', 'c', 'a', 't', 'i', 'o', 'n', '\0',
  /*  4220 - "dom" */ 'd', 'o', 'm', '\0',
  /*  4224 - "jsdebugger" */ 'j', 's', 'd', 'e', 'b', 'u', 'g', 'g', 'e', 'r', '\0',
  /*  4235 - "memory" */ 'm', 'e', 'm', 'o', 'r', 'y', '\0',
  /*  4242 - "options" */ 'o', 'p', 't', 'i', 'o', 'n', 's', '\0',
  /*  4250 - "performance" */ 'p', 'e', 'r', 'f', 'o', 'r', 'm', 'a', 'n', 'c', 'e', '\0',
  /*  4262 - "storage" */ 's', 't', 'o', 'r', 'a', 'g', 'e', '\0',
  /*  4270 - "styleeditor" */ 's', 't', 'y', 'l', 'e', 'e', 'd', 'i', 't', 'o', 'r', '\0',
  /*  4282 - "webconsole" */ 'w', 'e', 'b', 'c', 'o', 'n', 's', 'o', 'l', 'e', '\0',
  /*  4293 - "whatsnew" */ 'w', 'h', 'a', 't', 's', 'n', 'e', 'w', '\0',
  /*  4302 - "fakeTool4242" */ 'f', 'a', 'k', 'e', 'T', 'o', 'o', 'l', '4', '2', '4', '2', '\0',
  /*  4315 - "testBlankPanel" */ 't', 'e', 's', 't', 'B', 'l', 'a', 'n', 'k', 'P', 'a', 'n', 'e', 'l', '\0',
  /*  4330 - "testTool" */ 't', 'e', 's', 't', 'T', 'o', 'o', 'l', '\0',
  /*  4339 - "testtool1" */ 't', 'e', 's', 't', 't', 'o', 'o', 'l', '1', '\0',
  /*  4349 - "testTool1072208" */ 't', 'e', 's', 't', 'T', 'o', 'o', 'l', '1', '0', '7', '2', '2', '0', '8', '\0',
  /*  4365 - "testtool2" */ 't', 'e', 's', 't', 't', 'o', 'o', 'l', '2', '\0',
  /*  4375 - "execute_js" */ 'e', 'x', 'e', 'c', 'u', 't', 'e', '_', 'j', 's', '\0',
  /*  4386 - "exit" */ 'e', 'x', 'i', 't', '\0',
  /*  4391 - "f12_enabled" */ 'f', '1', '2', '_', 'e', 'n', 'a', 'b', 'l', 'e', 'd', '\0',
  /*  4403 - "f12_popup_displayed" */ 'f', '1', '2', '_', 'p', 'o', 'p', 'u', 'p', '_', 'd', 'i', 's', 'p', 'l', 'a', 'y', 'e', 'd', '\0',
  /*  4423 - "filters_changed" */ 'f', 'i', 'l', 't', 'e', 'r', 's', '_', 'c', 'h', 'a', 'n', 'g', 'e', 'd', '\0',
  /*  4439 - "inspect" */ 'i', 'n', 's', 'p', 'e', 'c', 't', '\0',
  /*  4447 - "jump_to_definition" */ 'j', 'u', 'm', 'p', '_', 't', 'o', '_', 'd', 'e', 'f', 'i', 'n', 'i', 't', 'i', 'o', 'n', '\0',
  /*  4466 - "jump_to_source" */ 'j', 'u', 'm', 'p', '_', 't', 'o', '_', 's', 'o', 'u', 'r', 'c', 'e', '\0',
  /*  4481 - "object_expanded" */ 'o', 'b', 'j', 'e', 'c', 't', '_', 'e', 'x', 'p', 'a', 'n', 'd', 'e', 'd', '\0',
  /*  4497 - "open" */ 'o', 'p', 'e', 'n', '\0',
  /*  4502 - "open_adbg" */ 'o', 'p', 'e', 'n', '_', 'a', 'd', 'b', 'g', '\0',
  /*  4512 - "pause" */ 'p', 'a', 'u', 's', 'e', '\0',
  /*  4518 - "pause_on_exceptions" */ 'p', 'a', 'u', 's', 'e', '_', 'o', 'n', '_', 'e', 'x', 'c', 'e', 'p', 't', 'i', 'o', 'n', 's', '\0',
  /*  4538 - "persist_changed" */ 'p', 'e', 'r', 's', 'i', 's', 't', '_', 'c', 'h', 'a', 'n', 'g', 'e', 'd', '\0',
  /*  4554 - "pretty_print" */ 'p', 'r', 'e', 't', 't', 'y', '_', 'p', 'r', 'i', 'n', 't', '\0',
  /*  4567 - "remove_breakpoint" */ 'r', 'e', 'm', 'o', 'v', 'e', '_', 'b', 'r', 'e', 'a', 'k', 'p', 'o', 'i', 'n', 't', '\0',
  /*  4585 - "reverse_search" */ 'r', 'e', 'v', 'e', 'r', 's', 'e', '_', 's', 'e', 'a', 'r', 'c', 'h', '\0',
  /*  4600 - "runtime_added" */ 'r', 'u', 'n', 't', 'i', 'm', 'e', '_', 'a', 'd', 'd', 'e', 'd', '\0',
  /*  4614 - "runtime_connected" */ 'r', 'u', 'n', 't', 'i', 'm', 'e', '_', 'c', 'o', 'n', 'n', 'e', 'c', 't', 'e', 'd', '\0',
  /*  4632 - "runtime_disconnected" */ 'r', 'u', 'n', 't', 'i', 'm', 'e', '_', 'd', 'i', 's', 'c', 'o', 'n', 'n', 'e', 'c', 't', 'e', 'd', '\0',
  /*  4653 - "runtime_removed" */ 'r', 'u', 'n', 't', 'i', 'm', 'e', '_', 'r', 'e', 'm', 'o', 'v', 'e', 'd', '\0',
  /*  4669 - "select_page" */ 's', 'e', 'l', 'e', 'c', 't', '_', 'p', 'a', 'g', 'e', '\0',
  /*  4681 - "select_ws_frame" */ 's', 'e', 'l', 'e', 'c', 't', '_', 'w', 's', '_', 'f', 'r', 'a', 'm', 'e', '\0',
  /*  4697 - "show_profiler" */ 's', 'h', 'o', 'w', '_', 'p', 'r', 'o', 'f', 'i', 'l', 'e', 'r', '\0',
  /*  4711 - "sidepanel_changed" */ 's', 'i', 'd', 'e', 'p', 'a', 'n', 'e', 'l', '_', 'c', 'h', 'a', 'n', 'g', 'e', 'd', '\0',
  /*  4729 - "start_worker" */ 's', 't', 'a', 'r', 't', '_', 'w', 'o', 'r', 'k', 'e', 'r', '\0',
  /*  4742 - "throttle_changed" */ 't', 'h', 'r', 'o', 't', 't', 'l', 'e', '_', 'c', 'h', 'a', 'n', 'g', 'e', 'd', '\0',
  /*  4759 - "tool_timer" */ 't', 'o', 'o', 'l', '_', 't', 'i', 'm', 'e', 'r', '\0',
  /*  4770 - "animationinspector" */ 'a', 'n', 'i', 'm', 'a', 't', 'i', 'o', 'n', 'i', 'n', 's', 'p', 'e', 'c', 't', 'o', 'r', '\0',
  /*  4789 - "compatibilityview" */ 'c', 'o', 'm', 'p', 'a', 't', 'i', 'b', 'i', 'l', 'i', 't', 'y', 'v', 'i', 'e', 'w', '\0',
  /*  4807 - "computedview" */ 'c', 'o', 'm', 'p', 'u', 't', 'e', 'd', 'v', 'i', 'e', 'w', '\0',
  /*  4820 - "changesview" */ 'c', 'h', 'a', 'n', 'g', 'e', 's', 'v', 'i', 'e', 'w', '\0',
  /*  4832 - "fontinspector" */ 'f', 'o', 'n', 't', 'i', 'n', 's', 'p', 'e', 'c', 't', 'o', 'r', '\0',
  /*  4846 - "layoutview" */ 'l', 'a', 'y', 'o', 'u', 't', 'v', 'i', 'e', 'w', '\0',
  /*  4857 - "unregister_worker" */ 'u', 'n', 'r', 'e', 'g', 'i', 's', 't', 'e', 'r', '_', 'w', 'o', 'r', 'k', 'e', 'r', '\0',
  /*  4875 - "update_conn_prompt" */ 'u', 'p', 'd', 'a', 't', 'e', '_', 'c', 'o', 'n', 'n', '_', 'p', 'r', 'o', 'm', 'p', 't', '\0',
  /*  4894 - "evaluate" */ 'e', 'v', 'a', 'l', 'u', 'a', 't', 'e', '\0',
  /*  4903 - "heuristics" */ 'h', 'e', 'u', 'r', 'i', 's', 't', 'i', 'c', 's', '\0',
  /*  4914 - "evaluate_v2" */ 'e', 'v', 'a', 'l', 'u', 'a', 't', 'e', '_', 'v', '2', '\0',
  /*  4926 - "state" */ 's', 't', 'a', 't', 'e', '\0',
  /*  4932 - "manuallyDisabled" */ 'm', 'a', 'n', 'u', 'a', 'l', 'l', 'y', 'D', 'i', 's', 'a', 'b', 'l', 'e', 'd', '\0',
  /*  4949 - "policyDisabled" */ 'p', 'o', 'l', 'i', 'c', 'y', 'D', 'i', 's', 'a', 'b', 'l', 'e', 'd', '\0',
  /*  4964 - "uninstalled" */ 'u', 'n', 'i', 'n', 's', 't', 'a', 'l', 'l', 'e', 'd', '\0',
  /*  4976 - "UIOk" */ 'U', 'I', 'O', 'k', '\0',
  /*  4981 - "UIDisabled" */ 'U', 'I', 'D', 'i', 's', 'a', 'b', 'l', 'e', 'd', '\0',
  /*  4992 - "rollback" */ 'r', 'o', 'l', 'l', 'b', 'a', 'c', 'k', '\0',
  /*  5001 - "shutdown" */ 's', 'h', 'u', 't', 'd', 'o', 'w', 'n', '\0',
  /*  5010 - "added" */ 'a', 'd', 'd', 'e', 'd', '\0',
  /*  5016 - "fileExtension" */ 'f', 'i', 'l', 'e', 'E', 'x', 't', 'e', 'n', 's', 'i', 'o', 'n', '\0',
  /*  5030 - "migrateResult" */ 'm', 'i', 'g', 'r', 'a', 't', 'e', 'R', 'e', 's', 'u', 'l', 't', '\0',
  /*  5044 - "storageLocal" */ 's', 't', 'o', 'r', 'a', 'g', 'e', 'L', 'o', 'c', 'a', 'l', '\0',
  /*  5057 - "storageLocalError" */ 's', 't', 'o', 'r', 'a', 'g', 'e', 'L', 'o', 'c', 'a', 'l', 'E', 'r', 'r', 'o', 'r', '\0',
  /*  5075 - "get" */ 'g', 'e', 't', '\0',
  /*  5079 - "set" */ 's', 'e', 't', '\0',
  /*  5083 - "remove" */ 'r', 'e', 'm', 'o', 'v', 'e', '\0',
  /*  5090 - "clear" */ 'c', 'l', 'e', 'a', 'r', '\0',
  /*  5096 - "logins" */ 'l', 'o', 'g', 'i', 'n', 's', '\0',
  /*  5103 - "connect" */ 'c', 'o', 'n', 'n', 'e', 'c', 't', '\0',
  /*  5111 - "account" */ 'a', 'c', 'c', 'o', 'u', 'n', 't', '\0',
  /*  5119 - "disconnect" */ 'd', 'i', 's', 'c', 'o', 'n', 'n', 'e', 'c', 't', '\0',
  /*  5130 - "click" */ 'c', 'l', 'i', 'c', 'k', '\0',
  /*  5136 - "account_settings" */ 'a', 'c', 'c', 'o', 'u', 'n', 't', '_', 's', 'e', 't', 't', 'i', 'n', 'g', 's', '\0',
  /*  5153 - "cad" */ 'c', 'a', 'd', '\0',
  /*  5157 - "send_tab" */ 's', 'e', 'n', 'd', '_', 't', 'a', 'b', '\0',
  /*  5166 - "sync_now" */ 's', 'y', 'n', 'c', '_', 'n', 'o', 'w', '\0',
  /*  5175 - "sync_settings" */ 's', 'y', 'n', 'c', '_', 's', 'e', 't', 't', 'i', 'n', 'g', 's', '\0',
  /*  5189 - "sync_tabs" */ 's', 'y', 'n', 'c', '_', 't', 'a', 'b', 's', '\0',
  /*  5199 - "sync_tabs_sidebar" */ 's', 'y', 'n', 'c', '_', 't', 'a', 'b', 's', '_', 's', 'i', 'd', 'e', 'b', 'a', 'r', '\0',
  /*  5217 - "toolbar_icon" */ 't', 'o', 'o', 'l', 'b', 'a', 'r', '_', 'i', 'c', 'o', 'n', '\0',
  /*  5230 - "unver_sync_settings" */ 'u', 'n', 'v', 'e', 'r', '_', 's', 'y', 'n', 'c', '_', 's', 'e', 't', 't', 'i', 'n', 'g', 's', '\0',
  /*  5250 - "open_monitor" */ 'o', 'p', 'e', 'n', '_', 'm', 'o', 'n', 'i', 't', 'o', 'r', '\0',
  /*  5263 - "open_send" */ 'o', 'p', 'e', 'n', '_', 's', 'e', 'n', 'd', '\0',
  /*  5273 - "ignore" */ 'i', 'g', 'n', 'o', 'r', 'e', '\0',
  /*  5280 - "first_seen" */ 'f', 'i', 'r', 's', 't', '_', 's', 'e', 'e', 'n', '\0',
  /*  5291 - "full" */ 'f', 'u', 'l', 'l', '\0',
  /*  5296 - "stub" */ 's', 't', 'u', 'b', '\0',
  /*  5301 - "dialog" */ 'd', 'i', 'a', 'l', 'o', 'g', '\0',
  /*  5308 - "main" */ 'm', 'a', 'i', 'n', '\0',
  /*  5313 - "search" */ 's', 'e', 'a', 'r', 'c', 'h', '\0',
  /*  5320 - "reorder" */ 'r', 'e', 'o', 'r', 'd', 'e', 'r', '\0',
  /*  5328 - "apply" */ 'a', 'p', 'p', 'l', 'y', '\0',
  /*  5334 - "accept" */ 'a', 'c', 'c', 'e', 'p', 't', '\0',
  /*  5341 - "load" */ 'l', 'o', 'a', 'd', '\0',
  /*  5346 - "autofillprofiles" */ 'a', 'u', 't', 'o', 'f', 'i', 'l', 'l', 'p', 'r', 'o', 'f', 'i', 'l', 'e', 's', '\0',
  /*  5363 - "reach" */ 'r', 'e', 'a', 'c', 'h', '\0',
  /*  5369 - "cfr" */ 'c', 'f', 'r', '\0',
  /*  5373 - "whats_new_panel" */ 'w', 'h', 'a', 't', 's', '_', 'n', 'e', 'w', '_', 'p', 'a', 'n', 'e', 'l', '\0',
  /*  5389 - "moments_page" */ 'm', 'o', 'm', 'e', 'n', 't', 's', '_', 'p', 'a', 'g', 'e', '\0',
  /*  5402 - "snippets" */ 's', 'n', 'i', 'p', 'p', 'e', 't', 's', '\0',
  /*  5411 - "cfr_fxa" */ 'c', 'f', 'r', '_', 'f', 'x', 'a', '\0',
  /*  5419 - "targeting" */ 't', 'a', 'r', 'g', 'e', 't', 'i', 'n', 'g', '\0',
  /*  5429 - "attribute_error" */ 'a', 't', 't', 'r', 'i', 'b', 'u', 't', 'e', '_', 'e', 'r', 'r', 'o', 'r', '\0',
  /*  5445 - "attribute_timeout" */ 'a', 't', 't', 'r', 'i', 'b', 'u', 't', 'e', '_', 't', 'i', 'm', 'e', 'o', 'u', 't', '\0',
  /*  5463 - "about_home" */ 'a', 'b', 'o', 'u', 't', '_', 'h', 'o', 'm', 'e', '\0',
  /*  5474 - "about_newtab" */ 'a', 'b', 'o', 'u', 't', '_', 'n', 'e', 'w', 't', 'a', 'b', '\0',
  /*  5487 - "contextmenu" */ 'c', 'o', 'n', 't', 'e', 'x', 't', 'm', 'e', 'n', 'u', '\0',
  /*  5499 - "oneoff" */ 'o', 'n', 'e', 'o', 'f', 'f', '\0',
  /*  5506 - "suggestion" */ 's', 'u', 'g', 'g', 'e', 's', 't', 'i', 'o', 'n', '\0',
  /*  5517 - "alias" */ 'a', 'l', 'i', 'a', 's', '\0',
  /*  5523 - "searchbar" */ 's', 'e', 'a', 'r', 'c', 'h', 'b', 'a', 'r', '\0',
  /*  5533 - "urlbar_searchmode" */ 'u', 'r', 'l', 'b', 'a', 'r', '_', 's', 'e', 'a', 'r', 'c', 'h', 'm', 'o', 'd', 'e', '\0',
  /*  5551 - "webextension" */ 'w', 'e', 'b', 'e', 'x', 't', 'e', 'n', 's', 'i', 'o', 'n', '\0',
  /*  5564 - "trrConfirmation" */ 't', 'r', 'r', 'C', 'o', 'n', 'f', 'i', 'r', 'm', 'a', 't', 'i', 'o', 'n', '\0',
  /*  5580 - "enroll" */ 'e', 'n', 'r', 'o', 'l', 'l', '\0',
  /*  5587 - "preference_study" */ 'p', 'r', 'e', 'f', 'e', 'r', 'e', 'n', 'c', 'e', '_', 's', 't', 'u', 'd', 'y', '\0',
  /*  5604 - "addon_study" */ 'a', 'd', 'd', 'o', 'n', '_', 's', 't', 'u', 'd', 'y', '\0',
  /*  5616 - "preference_rollout" */ 'p', 'r', 'e', 'f', 'e', 'r', 'e', 'n', 'c', 'e', '_', 'r', 'o', 'l', 'l', 'o', 'u', 't', '\0',
  /*  5635 - "addon_rollout" */ 'a', 'd', 'd', 'o', 'n', '_', 'r', 'o', 'l', 'l', 'o', 'u', 't', '\0',
  /*  5649 - "enrollFailed" */ 'e', 'n', 'r', 'o', 'l', 'l', 'F', 'a', 'i', 'l', 'e', 'd', '\0',
  /*  5662 - "expose" */ 'e', 'x', 'p', 'o', 's', 'e', '\0',
  /*  5669 - "feature_study" */ 'f', 'e', 'a', 't', 'u', 'r', 'e', '_', 's', 't', 'u', 'd', 'y', '\0',
  /*  5683 - "graduate" */ 'g', 'r', 'a', 'd', 'u', 'a', 't', 'e', '\0',
  /*  5692 - "unenroll" */ 'u', 'n', 'e', 'n', 'r', 'o', 'l', 'l', '\0',
  /*  5701 - "preference_rollback" */ 'p', 'r', 'e', 'f', 'e', 'r', 'e', 'n', 'c', 'e', '_', 'r', 'o', 'l', 'l', 'b', 'a', 'c', 'k', '\0',
  /*  5721 - "addon_rollback" */ 'a', 'd', 'd', 'o', 'n', '_', 'r', 'o', 'l', 'l', 'b', 'a', 'c', 'k', '\0',
  /*  5736 - "unenrollFailed" */ 'u', 'n', 'e', 'n', 'r', 'o', 'l', 'l', 'F', 'a', 'i', 'l', 'e', 'd', '\0',
  /*  5751 - "updateFailed" */ 'u', 'p', 'd', 'a', 't', 'e', 'F', 'a', 'i', 'l', 'e', 'd', '\0',
  /*  5764 - "attribution" */ 'a', 't', 't', 'r', 'i', 'b', 'u', 't', 'i', 'o', 'n', '\0',
  /*  5776 - "success" */ 's', 'u', 'c', 'c', 'e', 's', 's', '\0',
  /*  5784 - "failure" */ 'f', 'a', 'i', 'l', 'u', 'r', 'e', '\0',
  /*  5792 - "abort" */ 'a', 'b', 'o', 'r', 't', '\0',
  /*  5798 - "newtab" */ 'n', 'e', 'w', 't', 'a', 'b', '\0',
  /*  5805 - "create" */ 'c', 'r', 'e', 'a', 't', 'e', '\0',
  /*  5812 - "player" */ 'p', 'l', 'a', 'y', 'e', 'r', '\0',
  /*  5819 - "move" */ 'm', 'o', 'v', 'e', '\0',
  /*  5824 - "resize" */ 'r', 'e', 's', 'i', 'z', 'e', '\0',
  /*  5831 - "autocomplete_field" */ 'a', 'u', 't', 'o', 'c', 'o', 'm', 'p', 'l', 'e', 't', 'e', '_', 'f', 'i', 'e', 'l', 'd', '\0',
  /*  5850 - "generatedpassword" */ 'g', 'e', 'n', 'e', 'r', 'a', 't', 'e', 'd', 'p', 'a', 's', 's', 'w', 'o', 'r', 'd', '\0',
  /*  5868 - "autocomplete_shown" */ 'a', 'u', 't', 'o', 'c', 'o', 'm', 'p', 'l', 'e', 't', 'e', '_', 's', 'h', 'o', 'w', 'n', '\0',
  /*  5887 - "doorhanger_submitted" */ 'd', 'o', 'o', 'r', 'h', 'a', 'n', 'g', 'e', 'r', '_', 's', 'u', 'b', 'm', 'i', 't', 't', 'e', 'd', '\0',
  /*  5908 - "filled_field_edited" */ 'f', 'i', 'l', 'l', 'e', 'd', '_', 'f', 'i', 'e', 'l', 'd', '_', 'e', 'd', 'i', 't', 'e', 'd', '\0',
  /*  5928 - "existing_login" */ 'e', 'x', 'i', 's', 't', 'i', 'n', 'g', '_', 'l', 'o', 'g', 'i', 'n', '\0',
  /*  5943 - "list" */ 'l', 'i', 's', 't', '\0',
  /*  5948 - "new_login" */ 'n', 'e', 'w', '_', 'l', 'o', 'g', 'i', 'n', '\0',
  /*  5958 - "password" */ 'p', 'a', 's', 's', 'w', 'o', 'r', 'd', '\0',
  /*  5967 - "username" */ 'u', 's', 'e', 'r', 'n', 'a', 'm', 'e', '\0',
  /*  5976 - "copy" */ 'c', 'o', 'p', 'y', '\0',
  /*  5981 - "dismiss_breach_alert" */ 'd', 'i', 's', 'm', 'i', 's', 's', '_', 'b', 'r', 'e', 'a', 'c', 'h', '_', 'a', 'l', 'e', 'r', 't', '\0',
  /*  6002 - "filter" */ 'f', 'i', 'l', 't', 'e', 'r', '\0',
  /*  6009 - "hide" */ 'h', 'i', 'd', 'e', '\0',
  /*  6014 - "learn_more_breach" */ 'l', 'e', 'a', 'r', 'n', '_', 'm', 'o', 'r', 'e', '_', 'b', 'r', 'e', 'a', 'c', 'h', '\0',
  /*  6032 - "learn_more_vuln" */ 'l', 'e', 'a', 'r', 'n', '_', 'm', 'o', 'r', 'e', '_', 'v', 'u', 'l', 'n', '\0',
  /*  6048 - "new" */ 'n', 'e', 'w', '\0',
  /*  6052 - "open_site" */ 'o', 'p', 'e', 'n', '_', 's', 'i', 't', 'e', '\0',
  /*  6062 - "select" */ 's', 'e', 'l', 'e', 'c', 't', '\0',
  /*  6069 - "sort" */ 's', 'o', 'r', 't', '\0',
  /*  6074 - "mgmt_menu_item_used" */ 'm', 'g', 'm', 't', '_', 'm', 'e', 'n', 'u', '_', 'i', 't', 'e', 'm', '_', 'u', 's', 'e', 'd', '\0',
  /*  6094 - "import_from_browser" */ 'i', 'm', 'p', 'o', 'r', 't', '_', 'f', 'r', 'o', 'm', '_', 'b', 'r', 'o', 'w', 's', 'e', 'r', '\0',
  /*  6114 - "import_from_csv" */ 'i', 'm', 'p', 'o', 'r', 't', '_', 'f', 'r', 'o', 'm', '_', 'c', 's', 'v', '\0',
  /*  6130 - "import_csv_complete" */ 'i', 'm', 'p', 'o', 'r', 't', '_', 'c', 's', 'v', '_', 'c', 'o', 'm', 'p', 'l', 'e', 't', 'e', '\0',
  /*  6150 - "export" */ 'e', 'x', 'p', 'o', 'r', 't', '\0',
  /*  6157 - "export_complete" */ 'e', 'x', 'p', 'o', 'r', 't', '_', 'c', 'o', 'm', 'p', 'l', 'e', 't', 'e', '\0',
  /*  6173 - "preferences" */ 'p', 'r', 'e', 'f', 'e', 'r', 'e', 'n', 'c', 'e', 's', '\0',
  /*  6185 - "open_management" */ 'o', 'p', 'e', 'n', '_', 'm', 'a', 'n', 'a', 'g', 'e', 'm', 'e', 'n', 't', '\0',
  /*  6201 - "aboutprotections" */ 'a', 'b', 'o', 'u', 't', 'p', 'r', 'o', 't', 'e', 'c', 't', 'i', 'o', 'n', 's', '\0',
  /*  6218 - "autocomplete" */ 'a', 'u', 't', 'o', 'c', 'o', 'm', 'p', 'l', 'e', 't', 'e', '\0',
  /*  6231 - "capturedoorhanger" */ 'c', 'a', 'p', 't', 'u', 'r', 'e', 'd', 'o', 'o', 'r', 'h', 'a', 'n', 'g', 'e', 'r', '\0',
  /*  6249 - "direct" */ 'd', 'i', 'r', 'e', 'c', 't', '\0',
  /*  6256 - "fxamenu" */ 'f', 'x', 'a', 'm', 'e', 'n', 'u', '\0',
  /*  6264 - "mainmenu" */ 'm', 'a', 'i', 'n', 'm', 'e', 'n', 'u', '\0',
  /*  6273 - "pageinfo" */ 'p', 'a', 'g', 'e', 'i', 'n', 'f', 'o', '\0',
  /*  6282 - "snippet" */ 's', 'n', 'i', 'p', 'p', 'e', 't', '\0',
  /*  6290 - "reauthenticate" */ 'r', 'e', 'a', 'u', 't', 'h', 'e', 'n', 't', 'i', 'c', 'a', 't', 'e', '\0',
  /*  6305 - "master_password" */ 'm', 'a', 's', 't', 'e', 'r', '_', 'p', 'a', 's', 's', 'w', 'o', 'r', 'd', '\0',
  /*  6321 - "os_auth" */ 'o', 's', '_', 'a', 'u', 't', 'h', '\0',
  /*  6329 - "saved_login_used" */ 's', 'a', 'v', 'e', 'd', '_', 'l', 'o', 'g', 'i', 'n', '_', 'u', 's', 'e', 'd', '\0',
  /*  6346 - "form_login" */ 'f', 'o', 'r', 'm', '_', 'l', 'o', 'g', 'i', 'n', '\0',
  /*  6357 - "form_password" */ 'f', 'o', 'r', 'm', '_', 'p', 'a', 's', 's', 'w', 'o', 'r', 'd', '\0',
  /*  6371 - "auth_login" */ 'a', 'u', 't', 'h', '_', 'l', 'o', 'g', 'i', 'n', '\0',
  /*  6382 - "prompt_login" */ 'p', 'r', 'o', 'm', 'p', 't', '_', 'l', 'o', 'g', 'i', 'n', '\0',
  /*  6395 - "on" */ 'o', 'n', '\0',
  /*  6398 - "off" */ 'o', 'f', 'f', '\0',
  /*  6402 - "evalUsage" */ 'e', 'v', 'a', 'l', 'U', 's', 'a', 'g', 'e', '\0',
  /*  6412 - "systemContext" */ 's', 'y', 's', 't', 'e', 'm', 'C', 'o', 'n', 't', 'e', 'x', 't', '\0',
  /*  6426 - "parentProcess" */ 'p', 'a', 'r', 'e', 'n', 't', 'P', 'r', 'o', 'c', 'e', 's', 's', '\0',
  /*  6440 - "fissionPrincipals" */ 'f', 'i', 's', 's', 'i', 'o', 'n', 'P', 'r', 'i', 'n', 'c', 'i', 'p', 'a', 'l', 's', '\0',
  /*  6458 - "contentParent" */ 'c', 'o', 'n', 't', 'e', 'n', 't', 'P', 'a', 'r', 'e', 'n', 't', '\0',
  /*  6472 - "javascriptLoad" */ 'j', 'a', 'v', 'a', 's', 'c', 'r', 'i', 'p', 't', 'L', 'o', 'a', 'd', '\0',
  /*  6487 - "unexpectedload" */ 'u', 'n', 'e', 'x', 'p', 'e', 'c', 't', 'e', 'd', 'l', 'o', 'a', 'd', '\0',
  /*  6502 - "systemprincipal" */ 's', 'y', 's', 't', 'e', 'm', 'p', 'r', 'i', 'n', 'c', 'i', 'p', 'a', 'l', '\0',
  /*  6518 - "resolved" */ 'r', 'e', 's', 'o', 'l', 'v', 'e', 'd', '\0',
  /*  6527 - "record" */ 'r', 'e', 'c', 'o', 'r', 'd', '\0',
  /*  6534 - "trrselect" */ 't', 'r', 'r', 's', 'e', 'l', 'e', 'c', 't', '\0',
  /*  6544 - "dryrunresult" */ 'd', 'r', 'y', 'r', 'u', 'n', 'r', 'e', 's', 'u', 'l', 't', '\0',
  /*  6557 - "open_full_report" */ 'o', 'p', 'e', 'n', '_', 'f', 'u', 'l', 'l', '_', 'r', 'e', 'p', 'o', 'r', 't', '\0',
  /*  6574 - "advanced_button" */ 'a', 'd', 'v', 'a', 'n', 'c', 'e', 'd', '_', 'b', 'u', 't', 't', 'o', 'n', '\0',
  /*  6590 - "exception_button" */ 'e', 'x', 'c', 'e', 'p', 't', 'i', 'o', 'n', '_', 'b', 'u', 't', 't', 'o', 'n', '\0',
  /*  6607 - "return_button_top" */ 'r', 'e', 't', 'u', 'r', 'n', '_', 'b', 'u', 't', 't', 'o', 'n', '_', 't', 'o', 'p', '\0',
  /*  6625 - "return_button_adv" */ 'r', 'e', 't', 'u', 'r', 'n', '_', 'b', 'u', 't', 't', 'o', 'n', '_', 'a', 'd', 'v', '\0',
  /*  6643 - "learn_more_link" */ 'l', 'e', 'a', 'r', 'n', '_', 'm', 'o', 'r', 'e', '_', 'l', 'i', 'n', 'k', '\0',
  /*  6659 - "auto_report_cb" */ 'a', 'u', 't', 'o', '_', 'r', 'e', 'p', 'o', 'r', 't', '_', 'c', 'b', '\0',
  /*  6674 - "error_code_link" */ 'e', 'r', 'r', 'o', 'r', '_', 'c', 'o', 'd', 'e', '_', 'l', 'i', 'n', 'k', '\0',
  /*  6690 - "clipboard_button_top" */ 'c', 'l', 'i', 'p', 'b', 'o', 'a', 'r', 'd', '_', 'b', 'u', 't', 't', 'o', 'n', '_', 't', 'o', 'p', '\0',
  /*  6711 - "clipboard_button_bot" */ 'c', 'l', 'i', 'p', 'b', 'o', 'a', 'r', 'd', '_', 'b', 'u', 't', 't', 'o', 'n', '_', 'b', 'o', 't', '\0',
  /*  6732 - "aboutcerterror" */ 'a', 'b', 'o', 'u', 't', 'c', 'e', 'r', 't', 'e', 'r', 'r', 'o', 'r', '\0',
  /*  6747 - "lw_open_button" */ 'l', 'w', '_', 'o', 'p', 'e', 'n', '_', 'b', 'u', 't', 't', 'o', 'n', '\0',
  /*  6762 - "lw_sync_link" */ 'l', 'w', '_', 's', 'y', 'n', 'c', '_', 'l', 'i', 'n', 'k', '\0',
  /*  6775 - "lw_about_link" */ 'l', 'w', '_', 'a', 'b', 'o', 'u', 't', '_', 'l', 'i', 'n', 'k', '\0',
  /*  6789 - "mtr_about_link" */ 'm', 't', 'r', '_', 'a', 'b', 'o', 'u', 't', '_', 'l', 'i', 'n', 'k', '\0',
  /*  6804 - "mtr_report_link" */ 'm', 't', 'r', '_', 'r', 'e', 'p', 'o', 'r', 't', '_', 'l', 'i', 'n', 'k', '\0',
  /*  6820 - "mtr_signup_button" */ 'm', 't', 'r', '_', 's', 'i', 'g', 'n', 'u', 'p', '_', 'b', 'u', 't', 't', 'o', 'n', '\0',
  /*  6838 - "trackers_about_link" */ 't', 'r', 'a', 'c', 'k', 'e', 'r', 's', '_', 'a', 'b', 'o', 'u', 't', '_', 'l', 'i', 'n', 'k', '\0',
  /*  6858 - "mobile_app_link" */ 'm', 'o', 'b', 'i', 'l', 'e', '_', 'a', 'p', 'p', '_', 'l', 'i', 'n', 'k', '\0',
  /*  6874 - "settings_link" */ 's', 'e', 't', 't', 'i', 'n', 'g', 's', '_', 'l', 'i', 'n', 'k', '\0',
  /*  6888 - "vpn_banner_link" */ 'v', 'p', 'n', '_', 'b', 'a', 'n', 'n', 'e', 'r', '_', 'l', 'i', 'n', 'k', '\0',
  /*  6904 - "vpn_banner_close" */ 'v', 'p', 'n', '_', 'b', 'a', 'n', 'n', 'e', 'r', '_', 'c', 'l', 'o', 's', 'e', '\0',
  /*  6921 - "vpn_card_link" */ 'v', 'p', 'n', '_', 'c', 'a', 'r', 'd', '_', 'l', 'i', 'n', 'k', '\0',
  /*  6935 - "vpn_app_link_android" */ 'v', 'p', 'n', '_', 'a', 'p', 'p', '_', 'l', 'i', 'n', 'k', '_', 'a', 'n', 'd', 'r', 'o', 'i', 'd', '\0',
  /*  6956 - "vpn_app_link_ios" */ 'v', 'p', 'n', '_', 'a', 'p', 'p', '_', 'l', 'i', 'n', 'k', '_', 'i', 'o', 's', '\0',
  /*  6973 - "protection_report" */ 'p', 'r', 'o', 't', 'e', 'c', 't', 'i', 'o', 'n', '_', 'r', 'e', 'p', 'o', 'r', 't', '\0',
  /*  6991 - "vpn_banner" */ 'v', 'p', 'n', '_', 'b', 'a', 'n', 'n', 'e', 'r', '\0',
  /*  7002 - "etp_toggle_on" */ 'e', 't', 'p', '_', 't', 'o', 'g', 'g', 'l', 'e', '_', 'o', 'n', '\0',
  /*  7016 - "etp_toggle_off" */ 'e', 't', 'p', '_', 't', 'o', 'g', 'g', 'l', 'e', '_', 'o', 'f', 'f', '\0',
  /*  7031 - "sitenotworking_link" */ 's', 'i', 't', 'e', 'n', 'o', 't', 'w', 'o', 'r', 'k', 'i', 'n', 'g', '_', 'l', 'i', 'n', 'k', '\0',
  /*  7051 - "send_report_link" */ 's', 'e', 'n', 'd', '_', 'r', 'e', 'p', 'o', 'r', 't', '_', 'l', 'i', 'n', 'k', '\0',
  /*  7068 - "send_report_submit" */ 's', 'e', 'n', 'd', '_', 'r', 'e', 'p', 'o', 'r', 't', '_', 's', 'u', 'b', 'm', 'i', 't', '\0',
  /*  7087 - "social" */ 's', 'o', 'c', 'i', 'a', 'l', '\0',
  /*  7094 - "cookies" */ 'c', 'o', 'o', 'k', 'i', 'e', 's', '\0',
  /*  7102 - "trackers" */ 't', 'r', 'a', 'c', 'k', 'e', 'r', 's', '\0',
  /*  7111 - "fingerprinters" */ 'f', 'i', 'n', 'g', 'e', 'r', 'p', 'r', 'i', 'n', 't', 'e', 'r', 's', '\0',
  /*  7126 - "cryptominers" */ 'c', 'r', 'y', 'p', 't', 'o', 'm', 'i', 'n', 'e', 'r', 's', '\0',
  /*  7139 - "subview_settings" */ 's', 'u', 'b', 'v', 'i', 'e', 'w', '_', 's', 'e', 't', 't', 'i', 'n', 'g', 's', '\0',
  /*  7156 - "settings" */ 's', 'e', 't', 't', 'i', 'n', 'g', 's', '\0',
  /*  7165 - "full_report" */ 'f', 'u', 'l', 'l', '_', 'r', 'e', 'p', 'o', 'r', 't', '\0',
  /*  7177 - "milestone_message" */ 'm', 'i', 'l', 'e', 's', 't', 'o', 'n', 'e', '_', 'm', 'e', 's', 's', 'a', 'g', 'e', '\0',
  /*  7195 - "protections_popup" */ 'p', 'r', 'o', 't', 'e', 'c', 't', 'i', 'o', 'n', 's', '_', 'p', 'o', 'p', 'u', 'p', '\0',
  /*  7213 - "shown" */ 's', 'h', 'o', 'w', 'n', '\0',
  /*  7219 - "browser" */ 'b', 'r', 'o', 'w', 's', 'e', 'r', '\0',
  /*  7227 - "content" */ 'c', 'o', 'n', 't', 'e', 'n', 't', '\0',
  /*  7235 - "content_only" */ 'c', 'o', 'n', 't', 'e', 'n', 't', '_', 'o', 'n', 'l', 'y', '\0',
  /*  7248 - "object1" */ 'o', 'b', 'j', 'e', 'c', 't', '1', '\0',
  /*  7256 - "default_products" */ 'd', 'e', 'f', 'a', 'u', 'l', 't', '_', 'p', 'r', 'o', 'd', 'u', 'c', 't', 's', '\0',
  /*  7273 - "desktop_only" */ 'd', 'e', 's', 'k', 't', 'o', 'p', '_', 'o', 'n', 'l', 'y', '\0',
  /*  7286 - "expired_version" */ 'e', 'x', 'p', 'i', 'r', 'e', 'd', '_', 'v', 'e', 'r', 's', 'i', 'o', 'n', '\0',
  /*  7302 - "object2" */ 'o', 'b', 'j', 'e', 'c', 't', '2', '\0',
  /*  7310 - "main_and_content" */ 'm', 'a', 'i', 'n', '_', 'a', 'n', 'd', '_', 'c', 'o', 'n', 't', 'e', 'n', 't', '\0',
  /*  7327 - "main_only" */ 'm', 'a', 'i', 'n', '_', 'o', 'n', 'l', 'y', '\0',
  /*  7337 - "mobile_only" */ 'm', 'o', 'b', 'i', 'l', 'e', '_', 'o', 'n', 'l', 'y', '\0',
  /*  7349 - "multiproduct" */ 'm', 'u', 'l', 't', 'i', 'p', 'r', 'o', 'd', 'u', 'c', 't', '\0',
  /*  7362 - "not_expired_optout" */ 'n', 'o', 't', '_', 'e', 'x', 'p', 'i', 'r', 'e', 'd', '_', 'o', 'p', 't', 'o', 'u', 't', '\0',
  /*  7381 - "optout" */ 'o', 'p', 't', 'o', 'u', 't', '\0',
  /*  7388 - "test1" */ 't', 'e', 's', 't', '1', '\0',
  /*  7394 - "test2" */ 't', 'e', 's', 't', '2', '\0',
  /*  7400 - "test" */ 't', 'e', 's', 't', '\0',
  /*  7405 - "object3" */ 'o', 'b', 'j', 'e', 'c', 't', '3', '\0',
  /*  7413 - "uptake" */ 'u', 'p', 't', 'a', 'k', 'e', '\0',
  /*  7420 - "remotesettings" */ 'r', 'e', 'm', 'o', 't', 'e', 's', 'e', 't', 't', 'i', 'n', 'g', 's', '\0',
  /*  7435 - "abandonment" */ 'a', 'b', 'a', 'n', 'd', 'o', 'n', 'm', 'e', 'n', 't', '\0',
  /*  7447 - "blur" */ 'b', 'l', 'u', 'r', '\0',
  /*  7452 - "engagement" */ 'e', 'n', 'g', 'a', 'g', 'e', 'm', 'e', 'n', 't', '\0',
  /*  7463 - "paste_go" */ 'p', 'a', 's', 't', 'e', '_', 'g', 'o', '\0',
  /*  7472 - "drop_go" */ 'd', 'r', 'o', 'p', '_', 'g', 'o', '\0',
  /*  7480 - "share_display" */ 's', 'h', 'a', 'r', 'e', '_', 'd', 'i', 's', 'p', 'l', 'a', 'y', '\0',
  /*  7494 - "screen" */ 's', 'c', 'r', 'e', 'e', 'n', '\0',
  /*  7501 - "window" */ 'w', 'i', 'n', 'd', 'o', 'w', '\0',
  /*  7508 - "browser_window" */ 'b', 'r', 'o', 'w', 's', 'e', 'r', '_', 'w', 'i', 'n', 'd', 'o', 'w', '\0',
  /*  7523 - "show_indicator" */ 's', 'h', 'o', 'w', '_', 'i', 'n', 'd', 'i', 'c', 'a', 't', 'o', 'r', '\0',
  /*  7538 - "ftl" */ 'f', 't', 'l', '\0',
  /*  7542 - "dtd" */ 'd', 't', 'd', '\0',
  /*  7546 - "properties" */ 'p', 'r', 'o', 'p', 'e', 'r', 't', 'i', 'e', 's', '\0',
  /*  7557 - "js" */ 'j', 's', '\0',
  /*  7560 - "xml" */ 'x', 'm', 'l', '\0',
  /*  7564 - "xhtml" */ 'x', 'h', 't', 'm', 'l', '\0',
  /*  7570 - "css" */ 'c', 's', 's', '\0',
  /*  7574 - "json" */ 'j', 's', 'o', 'n', '\0',
  /*  7579 - "html" */ 'h', 't', 'm', 'l', '\0',
  /*  7584 - "png" */ 'p', 'n', 'g', '\0',
  /*  7588 - "svg" */ 's', 'v', 'g', '\0',
  /*  7592 - "others" */ 'o', 't', 'h', 'e', 'r', 's', '\0',
};

static_assert(sizeof(gEventsStringTable) <= UINT32_MAX, "index overflow");

} // namespace
#endif // mozilla_TelemetryEventData_h

