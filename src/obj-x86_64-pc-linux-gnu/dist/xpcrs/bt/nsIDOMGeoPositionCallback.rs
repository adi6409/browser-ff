//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/geolocation/nsIDOMGeoPositionCallback.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMGeoPositionCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void handleEvent (in nsIDOMGeoPosition position); */
                    Method {
                        name: "HandleEvent",
                        params: &[Param { name: "position", ty: "*const nsIDOMGeoPosition" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

