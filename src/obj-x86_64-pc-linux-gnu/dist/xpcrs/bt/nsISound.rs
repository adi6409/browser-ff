//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsISound.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISound",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void play (in nsIURL aURL); */
                    Method {
                        name: "Play",
                        params: &[Param { name: "aURL", ty: "*const nsIURL" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void beep (); */
                    Method {
                        name: "Beep",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void init (); */
                    Method {
                        name: "Init",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void playEventSound (in unsigned long aEventId); */
                    Method {
                        name: "PlayEventSound",
                        params: &[Param { name: "aEventId", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

