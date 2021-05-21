//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationControlService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITCPDeviceInfo",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AUTF8String id; */
                    Method {
                        name: "GetId",
                        params: &[Param { name: "aId", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String address; */
                    Method {
                        name: "GetAddress",
                        params: &[Param { name: "aAddress", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint16_t port; */
                    Method {
                        name: "GetPort",
                        params: &[Param { name: "aPort", ty: "*mut uint16_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String certFingerprint; */
                    Method {
                        name: "GetCertFingerprint",
                        params: &[Param { name: "aCertFingerprint", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPresentationControlServerListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onServerReady (in uint16_t aPort, in AUTF8String aCertFingerprint); */
                    Method {
                        name: "OnServerReady",
                        params: &[Param { name: "aPort", ty: "uint16_t" }, Param { name: "aCertFingerprint", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onServerStopped (in nsresult aResult); */
                    Method {
                        name: "OnServerStopped",
                        params: &[Param { name: "aResult", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onSessionRequest (in nsITCPDeviceInfo aDeviceInfo, in AString aUrl, in AString aPresentationId, in nsIPresentationControlChannel aControlChannel); */
                    Method {
                        name: "OnSessionRequest",
                        params: &[Param { name: "aDeviceInfo", ty: "*const nsITCPDeviceInfo" }, Param { name: "aUrl", ty: "*const ::nsstring::nsAString" }, Param { name: "aPresentationId", ty: "*const ::nsstring::nsAString" }, Param { name: "aControlChannel", ty: "*const nsIPresentationControlChannel" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onTerminateRequest (in nsITCPDeviceInfo aDeviceInfo, in AString aPresentationId, in nsIPresentationControlChannel aControlChannel, in boolean aIsFromReceiver); */
                    Method {
                        name: "OnTerminateRequest",
                        params: &[Param { name: "aDeviceInfo", ty: "*const nsITCPDeviceInfo" }, Param { name: "aPresentationId", ty: "*const ::nsstring::nsAString" }, Param { name: "aControlChannel", ty: "*const nsIPresentationControlChannel" }, Param { name: "aIsFromReceiver", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onReconnectRequest (in nsITCPDeviceInfo aDeviceInfo, in AString url, in AString aPresentationId, in nsIPresentationControlChannel aControlChannel); */
                    Method {
                        name: "OnReconnectRequest",
                        params: &[Param { name: "aDeviceInfo", ty: "*const nsITCPDeviceInfo" }, Param { name: "url", ty: "*const ::nsstring::nsAString" }, Param { name: "aPresentationId", ty: "*const ::nsstring::nsAString" }, Param { name: "aControlChannel", ty: "*const nsIPresentationControlChannel" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPresentationControlService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void startServer (in boolean aEncrypted, [optional] in uint16_t aPort); */
                    Method {
                        name: "StartServer",
                        params: &[Param { name: "aEncrypted", ty: "bool" }, Param { name: "aPort", ty: "uint16_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIPresentationControlChannel connect (in nsITCPDeviceInfo aDeviceInfo); */
                    Method {
                        name: "Connect",
                        params: &[Param { name: "aDeviceInfo", ty: "*const nsITCPDeviceInfo" }, Param { name: "_retval", ty: "*mut*const nsIPresentationControlChannel" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isCompatibleServer (in uint32_t aVersion); */
                    Method {
                        name: "IsCompatibleServer",
                        params: &[Param { name: "aVersion", ty: "uint32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void close (); */
                    Method {
                        name: "Close",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint16_t port; */
                    Method {
                        name: "GetPort",
                        params: &[Param { name: "aPort", ty: "*mut uint16_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint32_t version; */
                    Method {
                        name: "GetVersion",
                        params: &[Param { name: "aVersion", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AUTF8String id; */
                    Method {
                        name: "GetId",
                        params: &[Param { name: "aId", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetId",
                        params: &[Param { name: "aId", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AUTF8String certFingerprint; */
                    Method {
                        name: "GetCertFingerprint",
                        params: &[Param { name: "aCertFingerprint", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetCertFingerprint",
                        params: &[Param { name: "aCertFingerprint", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIPresentationControlServerListener listener; */
                    Method {
                        name: "GetListener",
                        params: &[Param { name: "aListener", ty: "*mut *const nsIPresentationControlServerListener" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetListener",
                        params: &[Param { name: "aListener", ty: "*const nsIPresentationControlServerListener" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

