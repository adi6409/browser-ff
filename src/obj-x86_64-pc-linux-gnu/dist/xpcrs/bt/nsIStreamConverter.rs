//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/streamconv/nsIStreamConverter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStreamConverter",
            base: Some("nsIStreamListener"),
            methods: Ok(&[
                    /* nsIInputStream convert (in nsIInputStream aFromStream, in string aFromType, in string aToType, in nsISupports aCtxt); */
                    Method {
                        name: "Convert",
                        params: &[Param { name: "aFromStream", ty: "*const nsIInputStream" }, Param { name: "aFromType", ty: "*const libc::c_char" }, Param { name: "aToType", ty: "*const libc::c_char" }, Param { name: "aCtxt", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut*const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void asyncConvertData (in string aFromType, in string aToType, in nsIStreamListener aListener, in nsISupports aCtxt); */
                    Method {
                        name: "AsyncConvertData",
                        params: &[Param { name: "aFromType", ty: "*const libc::c_char" }, Param { name: "aToType", ty: "*const libc::c_char" }, Param { name: "aListener", ty: "*const nsIStreamListener" }, Param { name: "aCtxt", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString getConvertedType (in ACString aFromType, in nsIChannel aChannel); */
                    Method {
                        name: "GetConvertedType",
                        params: &[Param { name: "aFromType", ty: "*const ::nsstring::nsACString" }, Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

