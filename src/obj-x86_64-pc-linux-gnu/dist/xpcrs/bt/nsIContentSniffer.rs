//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIContentSniffer.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIContentSniffer",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* ACString getMIMETypeFromContent (in nsIRequest aRequest, [array, size_is (aLength), const] in octet aData, in unsigned long aLength); */
                    Method {
                        name: "GetMIMETypeFromContent",
                        params: &[Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aData", ty: "*const u8" }, Param { name: "aLength", ty: "u32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

