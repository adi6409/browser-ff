//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationDeviceManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPresentationDeviceManager",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute boolean deviceAvailable; */
                    Method {
                        name: "GetDeviceAvailable",
                        params: &[Param { name: "aDeviceAvailable", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addDeviceProvider (in nsIPresentationDeviceProvider provider); */
                    Method {
                        name: "AddDeviceProvider",
                        params: &[Param { name: "provider", ty: "*const nsIPresentationDeviceProvider" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeDeviceProvider (in nsIPresentationDeviceProvider provider); */
                    Method {
                        name: "RemoveDeviceProvider",
                        params: &[Param { name: "provider", ty: "*const nsIPresentationDeviceProvider" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void forceDiscovery (); */
                    Method {
                        name: "ForceDiscovery",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIArray getAvailableDevices ([optional] in nsIArray presentationUrls); */
                    Method {
                        name: "GetAvailableDevices",
                        params: &[Param { name: "presentationUrls", ty: "*const nsIArray" }, Param { name: "_retval", ty: "*mut*const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

