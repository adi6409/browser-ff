//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/wifi/nsIWifiMonitor.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWifiMonitor",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void startWatching (in nsIWifiListener aListener); */
                    Method {
                        name: "StartWatching",
                        params: &[Param { name: "aListener", ty: "*const nsIWifiListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void stopWatching (in nsIWifiListener aListener); */
                    Method {
                        name: "StopWatching",
                        params: &[Param { name: "aListener", ty: "*const nsIWifiListener" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

