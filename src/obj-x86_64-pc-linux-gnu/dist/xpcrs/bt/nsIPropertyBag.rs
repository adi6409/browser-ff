//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIPropertyBag.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPropertyBag",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsISimpleEnumerator enumerator; */
                    Method {
                        name: "GetEnumerator",
                        params: &[Param { name: "aEnumerator", ty: "*mut*const nsISimpleEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIVariant getProperty (in AString name); */
                    Method {
                        name: "GetProperty",
                        params: &[Param { name: "name", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut*const nsIVariant" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

