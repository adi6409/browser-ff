//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIStringEnumerator.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStringEnumeratorBase",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [binaryname(StringIterator),symbol] nsIJSEnumerator iterator (); */
                    Method {
                        name: "StringIterator",
                        params: &[Param { name: "_retval", ty: "*mut*const nsIJSEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIStringEnumerator",
            base: Some("nsIStringEnumeratorBase"),
            methods: Ok(&[
                    /* boolean hasMore (); */
                    Method {
                        name: "HasMore",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getNext (); */
                    Method {
                        name: "GetNext",
                        params: &[Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIUTF8StringEnumerator",
            base: Some("nsIStringEnumeratorBase"),
            methods: Ok(&[
                    /* boolean hasMore (); */
                    Method {
                        name: "HasMore",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AUTF8String getNext (); */
                    Method {
                        name: "GetNext",
                        params: &[Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

