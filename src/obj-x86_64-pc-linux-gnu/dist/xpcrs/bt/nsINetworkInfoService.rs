//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsINetworkInfoService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIListNetworkAddressesListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onListedNetworkAddresses (in Array<ACString> aAddressArray); */
                    Method {
                        name: "OnListedNetworkAddresses",
                        params: &[Param { name: "aAddressArray", ty: "*const thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onListNetworkAddressesFailed (); */
                    Method {
                        name: "OnListNetworkAddressesFailed",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIGetHostnameListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onGotHostname (in AUTF8String aHostname); */
                    Method {
                        name: "OnGotHostname",
                        params: &[Param { name: "aHostname", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onGetHostnameFailed (); */
                    Method {
                        name: "OnGetHostnameFailed",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsINetworkInfoService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void listNetworkAddresses (in nsIListNetworkAddressesListener aListener); */
                    Method {
                        name: "ListNetworkAddresses",
                        params: &[Param { name: "aListener", ty: "*const nsIListNetworkAddressesListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getHostname (in nsIGetHostnameListener aListener); */
                    Method {
                        name: "GetHostname",
                        params: &[Param { name: "aListener", ty: "*const nsIGetHostnameListener" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

