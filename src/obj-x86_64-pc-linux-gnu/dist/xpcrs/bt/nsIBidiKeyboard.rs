//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIBidiKeyboard.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIBidiKeyboard",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void reset (); */
                    Method {
                        name: "Reset",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isLangRTL (); */
                    Method {
                        name: "IsLangRTL",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean haveBidiKeyboards; */
                    Method {
                        name: "GetHaveBidiKeyboards",
                        params: &[Param { name: "aHaveBidiKeyboards", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

