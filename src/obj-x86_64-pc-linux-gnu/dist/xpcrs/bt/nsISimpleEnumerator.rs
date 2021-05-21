//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsISimpleEnumerator.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIJSEnumerator",
            base: Some("nsISupports"),
            methods: Err("jscontext is unsupported"),
        },

        Interface {
            name: "nsISimpleEnumeratorBase",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [symbol] nsIJSEnumerator iterator (); */
                    Method {
                        name: "Iterator",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIJSEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIJSEnumerator entries (in nsIIDRef aIface); */
                    Method {
                        name: "Entries",
                        params: &[Param { name: "aIface", ty: "*const nsIID" }, Param { name: "_retval", ty: "*mut *const nsIJSEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISimpleEnumerator",
            base: Some("nsISimpleEnumeratorBase"),
            methods: Ok(&[
                    /* boolean hasMoreElements (); */
                    Method {
                        name: "HasMoreElements",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISupports getNext (); */
                    Method {
                        name: "GetNext",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

