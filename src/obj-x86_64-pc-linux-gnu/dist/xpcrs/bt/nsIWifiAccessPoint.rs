//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/wifi/nsIWifiAccessPoint.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWifiAccessPoint",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute ACString mac; */
                    Method {
                        name: "GetMac",
                        params: &[Param { name: "aMac", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString ssid; */
                    Method {
                        name: "GetSsid",
                        params: &[Param { name: "aSsid", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString rawSSID; */
                    Method {
                        name: "GetRawSSID",
                        params: &[Param { name: "aRawSSID", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long signal; */
                    Method {
                        name: "GetSignal",
                        params: &[Param { name: "aSignal", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

