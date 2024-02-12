use paste;

macro_rules! redef_units {
    (
        $(@add: [$(#[$newouter:meta])+];)?
        $(#[$outer:meta])*
        $vis:vis enum $name:ident$(<$($gtoks:tt)+>)?
        $(
            where
                $($wtoks:tt)+
        )?
        {
            $($vartoks:tt)+
        }
    ) => {
        paste::paste! {
            redef_units! {
                @inner
                ometa: [$([$outer])*],
                newouter: [$($([$newouter])+)?],
                name: [$name],
                new: [[<$name Variants>]],
                vis: [$vis],
                gtoks: [$($($gtoks)+)?],
                wtoks: [$($($wtoks)+)?],
                vartoks: [$($vartoks)+],
                vars: [],
                unitvars: [],
            }
        }
    };
    // unit variants
    (
        @inner
        ometa: $ometa:tt,
        newouter: $newouter:tt,
        name: $name:tt,
        new: $new:tt,
        vis: $vis:tt,
        gtoks: $gtoks:tt,
        wtoks: $wtoks:tt,
        vartoks: [
            $(#[$imeta:meta])*
            $vvis:vis $varname:ident$(,$($rest:tt)*)?
        ],
        vars: [$($var:tt)*],
        unitvars: [$($uvar:tt)*],
    ) => {
        redef_units! {
            @inner
            ometa: $ometa,
            newouter: $newouter,
            name: $name,
            new: $new,
            vis: $vis,
            gtoks: $gtoks,
            wtoks: $wtoks,
            vartoks: [$($($rest)*)?],
            vars: [$($var)* [[$([$imeta])*] $vvis $varname]],
            unitvars: [$($uvar)* [[$([$imeta])*] $vvis $varname]],
        }
    };
    // tuple variants
    (
        @inner
        ometa: $ometa:tt,
        newouter: $newouter:tt,
        name: $name:tt,
        new: $new:tt,
        vis: $vis:tt,
        gtoks: $gtoks:tt,
        wtoks: $wtoks:tt,
        vartoks: [
            $(#[$imeta:meta])*
            $vvis:vis $varname:ident($($tupletoks:tt)+)$(,$($rest:tt)*)?
        ],
        vars: [$($var:tt)*],
        unitvars: [$($uvar:tt)*],
    ) => {
        redef_units! {
            @inner
            ometa: $ometa,
            newouter: $newouter,
            name: $name,
            new: $new,
            vis: $vis,
            gtoks: $gtoks,
            wtoks: $wtoks,
            vartoks: [$($($rest)*)?],
            vars: [$($var)* [[$([$imeta])*] $vvis $varname($($tupletoks)*)]],
            unitvars: [$($uvar)* [[$([$imeta])*] $vvis $varname]],
        }
    };
    // struct variants
    (
        @inner
        ometa: $ometa:tt,
        newouter: $newouter:tt,
        name: $name:tt,
        new: $new:tt,
        vis: $vis:tt,
        gtoks: $gtoks:tt,
        wtoks: $wtoks:tt,
        vartoks: [
            $(#[$imeta:meta])*
            $vvis:vis $varname:ident { $($structtoks:tt)+ }$(,$($rest:tt)*)?
        ],
        vars: [$($var:tt)*],
        unitvars: [$($uvar:tt)*],
    ) => {
        redef_units! {
            @inner
            ometa: $ometa,
            newouter: $newouter,
            name: $name,
            new: $new,
            vis: $vis,
            gtoks: $gtoks,
            wtoks: $wtoks,
            vartoks: [$($($rest)*)?],
            vars: [$($var)* [[$([$imeta])*] $vvis $varname{ $($structtoks)+ }]],
            unitvars: [$($uvar)* [[$([$imeta])*] $vvis $varname]],
        }
    };
    (
        @inner
        ometa: [$([$outer:meta])*],
        newouter: [$([$newouter:meta])*],
        name: [$oldname:ident],
        new: [$newname:ident],
        vis: [$vis:vis],
        gtoks: [$($($gtoks:tt)+)?],
        wtoks: [$($($wtoks:tt)+)?],
        vartoks: [],
        vars: [$([[$([$imeta:meta])*] $($vartoks:tt)+])+],
        unitvars: [$([[$([$uimeta:meta])*] $($uvartoks:tt)+])+],
    ) => {
        $(#[$outer])*
        $vis enum $oldname$(<$($gtoks)+>)?
        $(where $($wtoks)+)?
        {
            $(
                $(#[$imeta])*
                $($vartoks)+,
            )+
        }

        $(#[$outer])*
        $(#[$newouter])*
        $vis enum $newname$(<$($gtoks)+>)?
        $(where $($wtoks)+)?
        {
            $(
                $(#[$uimeta])*
                $($uvartoks)+,
            )+
        }
    };
}

pub(crate) use redef_units;
