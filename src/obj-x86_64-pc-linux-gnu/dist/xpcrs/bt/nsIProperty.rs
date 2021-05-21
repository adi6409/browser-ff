//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIProperty.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIProperty",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AString name; */
                    Method {
                        name: "GetName",
                        params: &[Param { name: "aName", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIVariant value; */
                    Method {
                        name: "GetValue",
                        params: &[Param { name: "aValue", ty: "*mut*const nsIVariant" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

