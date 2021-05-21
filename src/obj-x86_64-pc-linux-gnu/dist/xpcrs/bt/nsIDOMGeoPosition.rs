//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/geolocation/nsIDOMGeoPosition.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMGeoPosition",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute DOMTimeStamp timestamp; */
                    Method {
                        name: "GetTimestamp",
                        params: &[Param { name: "aTimestamp", ty: "*mut DOMTimeStamp" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIDOMGeoPositionCoords coords; */
                    Method {
                        name: "GetCoords",
                        params: &[Param { name: "aCoords", ty: "*mut *const nsIDOMGeoPositionCoords" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

