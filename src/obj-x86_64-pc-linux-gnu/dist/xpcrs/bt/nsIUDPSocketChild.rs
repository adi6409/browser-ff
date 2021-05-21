//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/network/interfaces/nsIUDPSocketChild.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUDPSocketInternal",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void callListenerOpened (); */
                    Method {
                        name: "CallListenerOpened",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void callListenerConnected (); */
                    Method {
                        name: "CallListenerConnected",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void callListenerClosed (); */
                    Method {
                        name: "CallListenerClosed",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void callListenerReceivedData (in AUTF8String host, in unsigned short port, in Array<uint8_t> data); */
                    Method {
                        name: "CallListenerReceivedData",
                        params: &[Param { name: "host", ty: "*const ::nsstring::nsACString" }, Param { name: "port", ty: "u16" }, Param { name: "data", ty: "*const thin_vec::ThinVec<uint8_t>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void callListenerError (in AUTF8String message, in AUTF8String filename, in uint32_t lineNumber); */
                    Method {
                        name: "CallListenerError",
                        params: &[Param { name: "message", ty: "*const ::nsstring::nsACString" }, Param { name: "filename", ty: "*const ::nsstring::nsACString" }, Param { name: "lineNumber", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

