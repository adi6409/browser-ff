//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/windowwatcher/nsIWindowWatcher.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWindowWatcher",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* mozIDOMWindowProxy openWindow (in mozIDOMWindowProxy aParent, in ACString aUrl, in ACString aName, in ACString aFeatures, in nsISupports aArguments); */
                    Method {
                        name: "OpenWindow",
                        params: &[Param { name: "aParent", ty: "*const mozIDOMWindowProxy" }, Param { name: "aUrl", ty: "*const ::nsstring::nsACString" }, Param { name: "aName", ty: "*const ::nsstring::nsACString" }, Param { name: "aFeatures", ty: "*const ::nsstring::nsACString" }, Param { name: "aArguments", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut*const mozIDOMWindowProxy" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void registerNotification (in nsIObserver aObserver); */
                    Method {
                        name: "RegisterNotification",
                        params: &[Param { name: "aObserver", ty: "*const nsIObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void unregisterNotification (in nsIObserver aObserver); */
                    Method {
                        name: "UnregisterNotification",
                        params: &[Param { name: "aObserver", ty: "*const nsIObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISimpleEnumerator getWindowEnumerator (); */
                    Method {
                        name: "GetWindowEnumerator",
                        params: &[Param { name: "_retval", ty: "*mut*const nsISimpleEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIPrompt getNewPrompter (in mozIDOMWindowProxy aParent); */
                    Method {
                        name: "GetNewPrompter",
                        params: &[Param { name: "aParent", ty: "*const mozIDOMWindowProxy" }, Param { name: "_retval", ty: "*mut*const nsIPrompt" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIAuthPrompt getNewAuthPrompter (in mozIDOMWindowProxy aParent); */
                    Method {
                        name: "GetNewAuthPrompter",
                        params: &[Param { name: "aParent", ty: "*const mozIDOMWindowProxy" }, Param { name: "_retval", ty: "*mut*const nsIAuthPrompt" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setWindowCreator (in nsIWindowCreator creator); */
                    Method {
                        name: "SetWindowCreator",
                        params: &[Param { name: "creator", ty: "*const nsIWindowCreator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean hasWindowCreator (); */
                    Method {
                        name: "HasWindowCreator",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIWebBrowserChrome getChromeForWindow (in mozIDOMWindowProxy aWindow); */
                    Method {
                        name: "GetChromeForWindow",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindowProxy" }, Param { name: "_retval", ty: "*mut*const nsIWebBrowserChrome" }],
                        ret: "::nserror::nsresult",
                    },

                    /* mozIDOMWindowProxy getWindowByName (in AString aTargetName, in mozIDOMWindowProxy aCurrentWindow); */
                    Method {
                        name: "GetWindowByName",
                        params: &[Param { name: "aTargetName", ty: "*const ::nsstring::nsAString" }, Param { name: "aCurrentWindow", ty: "*const mozIDOMWindowProxy" }, Param { name: "_retval", ty: "*mut*const mozIDOMWindowProxy" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute mozIDOMWindowProxy activeWindow; */
                    Method {
                        name: "GetActiveWindow",
                        params: &[Param { name: "aActiveWindow", ty: "*mut*const mozIDOMWindowProxy" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

