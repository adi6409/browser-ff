//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIDashboardEventNotifier.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDashboardEventNotifier",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void addHost (in ACString aHost, in unsigned long aSerial, in boolean aEncrypted); */
                    Method {
                        name: "AddHost",
                        params: &[Param { name: "aHost", ty: "*const ::nsstring::nsACString" }, Param { name: "aSerial", ty: "u32" }, Param { name: "aEncrypted", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeHost (in ACString aHost, in unsigned long aSerial); */
                    Method {
                        name: "RemoveHost",
                        params: &[Param { name: "aHost", ty: "*const ::nsstring::nsACString" }, Param { name: "aSerial", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void newMsgSent (in ACString aHost, in unsigned long aSerial, in unsigned long aLength); */
                    Method {
                        name: "NewMsgSent",
                        params: &[Param { name: "aHost", ty: "*const ::nsstring::nsACString" }, Param { name: "aSerial", ty: "u32" }, Param { name: "aLength", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void newMsgReceived (in ACString aHost, in unsigned long aSerial, in unsigned long aLength); */
                    Method {
                        name: "NewMsgReceived",
                        params: &[Param { name: "aHost", ty: "*const ::nsstring::nsACString" }, Param { name: "aSerial", ty: "u32" }, Param { name: "aLength", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

