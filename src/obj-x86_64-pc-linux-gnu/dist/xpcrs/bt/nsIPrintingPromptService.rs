//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/browser/nsIPrintingPromptService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPrintingPromptService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void showPrintDialog (in mozIDOMWindowProxy parent, in nsIPrintSettings printSettings); */
                    Method {
                        name: "ShowPrintDialog",
                        params: &[Param { name: "parent", ty: "*const mozIDOMWindowProxy" }, Param { name: "printSettings", ty: "*const nsIPrintSettings" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void showPrintProgressDialog (in mozIDOMWindowProxy parent, in nsIPrintSettings printSettings, in nsIObserver openDialogObserver, in boolean isForPrinting, out nsIWebProgressListener webProgressListener, out nsIPrintProgressParams printProgressParams, out boolean notifyOnOpen); */
                    Method {
                        name: "ShowPrintProgressDialog",
                        params: &[Param { name: "parent", ty: "*const mozIDOMWindowProxy" }, Param { name: "printSettings", ty: "*const nsIPrintSettings" }, Param { name: "openDialogObserver", ty: "*const nsIObserver" }, Param { name: "isForPrinting", ty: "bool" }, Param { name: "webProgressListener", ty: "*mut *const nsIWebProgressListener" }, Param { name: "printProgressParams", ty: "*mut *const nsIPrintProgressParams" }, Param { name: "notifyOnOpen", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void showPageSetupDialog (in mozIDOMWindowProxy parent, in nsIPrintSettings printSettings); */
                    Method {
                        name: "ShowPageSetupDialog",
                        params: &[Param { name: "parent", ty: "*const mozIDOMWindowProxy" }, Param { name: "printSettings", ty: "*const nsIPrintSettings" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

