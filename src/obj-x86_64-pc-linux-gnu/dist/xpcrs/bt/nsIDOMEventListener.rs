//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/events/nsIDOMEventListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMEventListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [can_run_script] void handleEvent (in Event event); */
                    Method {
                        name: "HandleEvent",
                        params: &[Param { name: "event", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

