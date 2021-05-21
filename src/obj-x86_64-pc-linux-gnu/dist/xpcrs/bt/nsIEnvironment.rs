//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/threads/nsIEnvironment.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIEnvironment",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void set (in AString aName, in AString aValue); */
                    Method {
                        name: "Set",
                        params: &[Param { name: "aName", ty: "*const ::nsstring::nsAString" }, Param { name: "aValue", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString get (in AString aName); */
                    Method {
                        name: "Get",
                        params: &[Param { name: "aName", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean exists (in AString aName); */
                    Method {
                        name: "Exists",
                        params: &[Param { name: "aName", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

