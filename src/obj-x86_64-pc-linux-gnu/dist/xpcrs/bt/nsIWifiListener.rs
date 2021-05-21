//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/wifi/nsIWifiListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWifiListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onChange (in Array<nsIWifiAccessPoint> accessPoints); */
                    Method {
                        name: "OnChange",
                        params: &[Param { name: "accessPoints", ty: "*const thin_vec::ThinVec<RefPtr<nsIWifiAccessPoint>>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onError (in nsresult error); */
                    Method {
                        name: "OnError",
                        params: &[Param { name: "error", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

