//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsICloneableInputStream.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICloneableInputStream",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [infallible] readonly attribute boolean cloneable; */
                    Method {
                        name: "GetCloneable",
                        params: &[Param { name: "aCloneable", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIInputStream clone (); */
                    Method {
                        name: "Clone",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsICloneableInputStreamWithRange",
            base: Some("nsICloneableInputStream"),
            methods: Ok(&[
                    /* nsIInputStream cloneWithRange (in uint64_t start, in uint64_t length); */
                    Method {
                        name: "CloneWithRange",
                        params: &[Param { name: "start", ty: "uint64_t" }, Param { name: "length", ty: "uint64_t" }, Param { name: "_retval", ty: "*mut *const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

