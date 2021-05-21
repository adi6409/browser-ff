//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsITooltipListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITooltipListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onShowTooltip (in long aXCoords, in long aYCoords, in AString aTipText, in AString aTipDir); */
                    Method {
                        name: "OnShowTooltip",
                        params: &[Param { name: "aXCoords", ty: "i32" }, Param { name: "aYCoords", ty: "i32" }, Param { name: "aTipText", ty: "*const ::nsstring::nsAString" }, Param { name: "aTipDir", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onHideTooltip (); */
                    Method {
                        name: "OnHideTooltip",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

