//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/dns/mdns/nsIDNSServiceDiscovery.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDNSServiceInfo",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute AUTF8String host; */
                    Method {
                        name: "GetHost",
                        params: &[Param { name: "aHost", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetHost",
                        params: &[Param { name: "aHost", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AUTF8String address; */
                    Method {
                        name: "GetAddress",
                        params: &[Param { name: "aAddress", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetAddress",
                        params: &[Param { name: "aAddress", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute unsigned short port; */
                    Method {
                        name: "GetPort",
                        params: &[Param { name: "aPort", ty: "*mut u16" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetPort",
                        params: &[Param { name: "aPort", ty: "u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AUTF8String serviceName; */
                    Method {
                        name: "GetServiceName",
                        params: &[Param { name: "aServiceName", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetServiceName",
                        params: &[Param { name: "aServiceName", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AUTF8String serviceType; */
                    Method {
                        name: "GetServiceType",
                        params: &[Param { name: "aServiceType", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetServiceType",
                        params: &[Param { name: "aServiceType", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AUTF8String domainName; */
                    Method {
                        name: "GetDomainName",
                        params: &[Param { name: "aDomainName", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDomainName",
                        params: &[Param { name: "aDomainName", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIPropertyBag2 attributes; */
                    Method {
                        name: "GetAttributes",
                        params: &[Param { name: "aAttributes", ty: "*mut*const nsIPropertyBag2" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetAttributes",
                        params: &[Param { name: "aAttributes", ty: "*const nsIPropertyBag2" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIDNSServiceDiscoveryListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onDiscoveryStarted (in AUTF8String aServiceType); */
                    Method {
                        name: "OnDiscoveryStarted",
                        params: &[Param { name: "aServiceType", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onDiscoveryStopped (in AUTF8String aServiceType); */
                    Method {
                        name: "OnDiscoveryStopped",
                        params: &[Param { name: "aServiceType", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onServiceFound (in nsIDNSServiceInfo aServiceInfo); */
                    Method {
                        name: "OnServiceFound",
                        params: &[Param { name: "aServiceInfo", ty: "*const nsIDNSServiceInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onServiceLost (in nsIDNSServiceInfo aServiceInfo); */
                    Method {
                        name: "OnServiceLost",
                        params: &[Param { name: "aServiceInfo", ty: "*const nsIDNSServiceInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onStartDiscoveryFailed (in AUTF8String aServiceType, in long aErrorCode); */
                    Method {
                        name: "OnStartDiscoveryFailed",
                        params: &[Param { name: "aServiceType", ty: "*const ::nsstring::nsACString" }, Param { name: "aErrorCode", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onStopDiscoveryFailed (in AUTF8String aServiceType, in long aErrorCode); */
                    Method {
                        name: "OnStopDiscoveryFailed",
                        params: &[Param { name: "aServiceType", ty: "*const ::nsstring::nsACString" }, Param { name: "aErrorCode", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIDNSRegistrationListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onServiceRegistered (in nsIDNSServiceInfo aServiceInfo); */
                    Method {
                        name: "OnServiceRegistered",
                        params: &[Param { name: "aServiceInfo", ty: "*const nsIDNSServiceInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onServiceUnregistered (in nsIDNSServiceInfo aServiceInfo); */
                    Method {
                        name: "OnServiceUnregistered",
                        params: &[Param { name: "aServiceInfo", ty: "*const nsIDNSServiceInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onRegistrationFailed (in nsIDNSServiceInfo aServiceInfo, in long aErrorCode); */
                    Method {
                        name: "OnRegistrationFailed",
                        params: &[Param { name: "aServiceInfo", ty: "*const nsIDNSServiceInfo" }, Param { name: "aErrorCode", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onUnregistrationFailed (in nsIDNSServiceInfo aServiceInfo, in long aErrorCode); */
                    Method {
                        name: "OnUnregistrationFailed",
                        params: &[Param { name: "aServiceInfo", ty: "*const nsIDNSServiceInfo" }, Param { name: "aErrorCode", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIDNSServiceResolveListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onServiceResolved (in nsIDNSServiceInfo aServiceInfo); */
                    Method {
                        name: "OnServiceResolved",
                        params: &[Param { name: "aServiceInfo", ty: "*const nsIDNSServiceInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onResolveFailed (in nsIDNSServiceInfo aServiceInfo, in long aErrorCode); */
                    Method {
                        name: "OnResolveFailed",
                        params: &[Param { name: "aServiceInfo", ty: "*const nsIDNSServiceInfo" }, Param { name: "aErrorCode", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIDNSServiceDiscovery",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsICancelable startDiscovery (in AUTF8String aServiceType, in nsIDNSServiceDiscoveryListener aListener); */
                    Method {
                        name: "StartDiscovery",
                        params: &[Param { name: "aServiceType", ty: "*const ::nsstring::nsACString" }, Param { name: "aListener", ty: "*const nsIDNSServiceDiscoveryListener" }, Param { name: "_retval", ty: "*mut*const nsICancelable" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsICancelable registerService (in nsIDNSServiceInfo aServiceInfo, in nsIDNSRegistrationListener aListener); */
                    Method {
                        name: "RegisterService",
                        params: &[Param { name: "aServiceInfo", ty: "*const nsIDNSServiceInfo" }, Param { name: "aListener", ty: "*const nsIDNSRegistrationListener" }, Param { name: "_retval", ty: "*mut*const nsICancelable" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void resolveService (in nsIDNSServiceInfo aServiceInfo, in nsIDNSServiceResolveListener aListener); */
                    Method {
                        name: "ResolveService",
                        params: &[Param { name: "aServiceInfo", ty: "*const nsIDNSServiceInfo" }, Param { name: "aListener", ty: "*const nsIDNSServiceResolveListener" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

