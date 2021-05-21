//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/exthandler/nsIHelperAppLauncherDialog.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHelperAppLauncherDialog",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void show (in nsIHelperAppLauncher aLauncher, in nsIInterfaceRequestor aWindowContext, in unsigned long aReason); */
                    Method {
                        name: "Show",
                        params: &[Param { name: "aLauncher", ty: "*const nsIHelperAppLauncher" }, Param { name: "aWindowContext", ty: "*const nsIInterfaceRequestor" }, Param { name: "aReason", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void promptForSaveToFileAsync (in nsIHelperAppLauncher aLauncher, in nsIInterfaceRequestor aWindowContext, in wstring aDefaultFileName, in wstring aSuggestedFileExtension, in boolean aForcePrompt); */
                    Method {
                        name: "PromptForSaveToFileAsync",
                        params: &[Param { name: "aLauncher", ty: "*const nsIHelperAppLauncher" }, Param { name: "aWindowContext", ty: "*const nsIInterfaceRequestor" }, Param { name: "aDefaultFileName", ty: "*const i16" }, Param { name: "aSuggestedFileExtension", ty: "*const i16" }, Param { name: "aForcePrompt", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

