//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/streamconv/nsIStreamConverterService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStreamConverterService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* boolean canConvert (in string aFromType, in string aToType); */
                    Method {
                        name: "CanConvert",
                        params: &[Param { name: "aFromType", ty: "*const libc::c_char" }, Param { name: "aToType", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString convertedType (in ACString aFromType, in nsIChannel aChannel); */
                    Method {
                        name: "ConvertedType",
                        params: &[Param { name: "aFromType", ty: "*const ::nsstring::nsACString" }, Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIInputStream convert (in nsIInputStream aFromStream, in string aFromType, in string aToType, in nsISupports aContext); */
                    Method {
                        name: "Convert",
                        params: &[Param { name: "aFromStream", ty: "*const nsIInputStream" }, Param { name: "aFromType", ty: "*const libc::c_char" }, Param { name: "aToType", ty: "*const libc::c_char" }, Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut*const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIStreamListener asyncConvertData (in string aFromType, in string aToType, in nsIStreamListener aListener, in nsISupports aContext); */
                    Method {
                        name: "AsyncConvertData",
                        params: &[Param { name: "aFromType", ty: "*const libc::c_char" }, Param { name: "aToType", ty: "*const libc::c_char" }, Param { name: "aListener", ty: "*const nsIStreamListener" }, Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut*const nsIStreamListener" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

