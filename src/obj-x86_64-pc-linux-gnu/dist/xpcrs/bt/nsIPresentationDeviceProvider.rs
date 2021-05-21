//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationDeviceProvider.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPresentationDeviceListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void addDevice (in nsIPresentationDevice device); */
                    Method {
                        name: "AddDevice",
                        params: &[Param { name: "device", ty: "*const nsIPresentationDevice" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeDevice (in nsIPresentationDevice device); */
                    Method {
                        name: "RemoveDevice",
                        params: &[Param { name: "device", ty: "*const nsIPresentationDevice" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void updateDevice (in nsIPresentationDevice device); */
                    Method {
                        name: "UpdateDevice",
                        params: &[Param { name: "device", ty: "*const nsIPresentationDevice" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onSessionRequest (in nsIPresentationDevice device, in AString url, in AString presentationId, in nsIPresentationControlChannel controlChannel); */
                    Method {
                        name: "OnSessionRequest",
                        params: &[Param { name: "device", ty: "*const nsIPresentationDevice" }, Param { name: "url", ty: "*const ::nsstring::nsAString" }, Param { name: "presentationId", ty: "*const ::nsstring::nsAString" }, Param { name: "controlChannel", ty: "*const nsIPresentationControlChannel" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onTerminateRequest (in nsIPresentationDevice device, in AString presentationId, in nsIPresentationControlChannel controlChannel, in boolean aIsFromReceiver); */
                    Method {
                        name: "OnTerminateRequest",
                        params: &[Param { name: "device", ty: "*const nsIPresentationDevice" }, Param { name: "presentationId", ty: "*const ::nsstring::nsAString" }, Param { name: "controlChannel", ty: "*const nsIPresentationControlChannel" }, Param { name: "aIsFromReceiver", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onReconnectRequest (in nsIPresentationDevice device, in AString url, in AString presentationId, in nsIPresentationControlChannel controlChannel); */
                    Method {
                        name: "OnReconnectRequest",
                        params: &[Param { name: "device", ty: "*const nsIPresentationDevice" }, Param { name: "url", ty: "*const ::nsstring::nsAString" }, Param { name: "presentationId", ty: "*const ::nsstring::nsAString" }, Param { name: "controlChannel", ty: "*const nsIPresentationControlChannel" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPresentationDeviceProvider",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute nsIPresentationDeviceListener listener; */
                    Method {
                        name: "GetListener",
                        params: &[Param { name: "aListener", ty: "*mut *const nsIPresentationDeviceListener" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetListener",
                        params: &[Param { name: "aListener", ty: "*const nsIPresentationDeviceListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void forceDiscovery (); */
                    Method {
                        name: "ForceDiscovery",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

