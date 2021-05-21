//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/xre/nsINativeAppSupport.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINativeAppSupport",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* boolean start (); */
                    Method {
                        name: "Start",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void enable (); */
                    Method {
                        name: "Enable",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void onLastWindowClosing (); */
                    Method {
                        name: "OnLastWindowClosing",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void ReOpen (); */
                    Method {
                        name: "ReOpen",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

