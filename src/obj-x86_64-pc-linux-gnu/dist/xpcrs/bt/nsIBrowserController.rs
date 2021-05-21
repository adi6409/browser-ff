//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/xul/nsIBrowserController.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIBrowserController",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void enableDisableCommands (in AString action, in Array<ACString> enabledCommands, in Array<ACString> disabledCommands); */
                    Method {
                        name: "EnableDisableCommands",
                        params: &[Param { name: "action", ty: "*const ::nsstring::nsAString" }, Param { name: "enabledCommands", ty: "*const thin_vec::ThinVec<::nsstring::nsCString>" }, Param { name: "disabledCommands", ty: "*const thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

