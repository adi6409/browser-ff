//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIUploadChannel2.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUploadChannel2",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void explicitSetUploadStream (in nsIInputStream aStream, in ACString aContentType, in long long aContentLength, in ACString aMethod, in boolean aStreamHasHeaders); */
                    Method {
                        name: "ExplicitSetUploadStream",
                        params: &[Param { name: "aStream", ty: "*const nsIInputStream" }, Param { name: "aContentType", ty: "*const ::nsstring::nsACString" }, Param { name: "aContentLength", ty: "i64" }, Param { name: "aMethod", ty: "*const ::nsstring::nsACString" }, Param { name: "aStreamHasHeaders", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean uploadStreamHasHeaders; */
                    Method {
                        name: "GetUploadStreamHasHeaders",
                        params: &[Param { name: "aUploadStreamHasHeaders", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] void ensureUploadStreamIsCloneable (in nsIRunnable aCallback); */
                    Method {
                        name: "EnsureUploadStreamIsCloneable",
                        params: &[Param { name: "aCallback", ty: "*const nsIRunnable" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] nsIInputStream cloneUploadStream (out long long aContentLength); */
                    Method {
                        name: "CloneUploadStream",
                        params: &[Param { name: "aContentLength", ty: "*mut i64" }, Param { name: "_retval", ty: "*mut*const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

