//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsIGeolocationProvider.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIGeolocationUpdate",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void update (in nsIDOMGeoPosition position); */
                    Method {
                        name: "Update",
                        params: &[Param { name: "position", ty: "*const nsIDOMGeoPosition" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void notifyError (in unsigned short error); */
                    Method {
                        name: "NotifyError",
                        params: &[Param { name: "error", ty: "u16" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIGeolocationProvider",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void startup (); */
                    Method {
                        name: "Startup",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void watch (in nsIGeolocationUpdate callback); */
                    Method {
                        name: "Watch",
                        params: &[Param { name: "callback", ty: "*const nsIGeolocationUpdate" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void shutdown (); */
                    Method {
                        name: "Shutdown",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void setHighAccuracy (in boolean enable); */
                    Method {
                        name: "SetHighAccuracy",
                        params: &[Param { name: "enable", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

