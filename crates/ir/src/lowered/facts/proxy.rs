use crate::builtin_resolved::ResolvedExpr;

#[derive(Debug, Clone)]
pub struct ProxyBinding {
    pub target: ResolvedExpr,
    pub handler: ResolvedExpr,
}

/// Statically visible Intl.NumberFormat constructor options.
#[derive(Debug, Clone)]
pub struct IntlNumberFormatOptions {
    pub locale: String,
    pub style: String,
    pub currency: String,
    pub notation: String,
    pub compact_display: String,
    pub sign_display: String,
}

/// Statically visible Intl.DateTimeFormat constructor options.
#[derive(Debug, Clone)]
pub struct IntlDateTimeFormatOptions {
    pub locale: String,
    pub time_zone: String,
    pub locale_matcher: String,
}

/// Static Proxy trap lowering kinds for the supported MVP trap slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProxyTrapKind {
    ProxyGet,
    ProxySet,
    ProxyHas,
    ProxyDeleteProperty,
    ProxyConstruct,
    ProxyApply,
    ProxyGetPrototypeOf,
    ProxySetPrototypeOf,
    ProxyIsExtensible,
    ProxyPreventExtensions,
    ProxyGetOwnPropertyDescriptor,
    ProxyDefineProperty,
    ProxyOwnKeys,
    Named(&'static str),
}

impl ProxyTrapKind {
    pub fn method_name(self) -> &'static str {
        match self {
            Self::ProxyGet => "get",
            Self::ProxySet => "set",
            Self::ProxyHas => "has",
            Self::ProxyDeleteProperty => "deleteProperty",
            Self::ProxyConstruct => "construct",
            Self::ProxyApply => "apply",
            Self::ProxyGetPrototypeOf => "getPrototypeOf",
            Self::ProxySetPrototypeOf => "setPrototypeOf",
            Self::ProxyIsExtensible => "isExtensible",
            Self::ProxyPreventExtensions => "preventExtensions",
            Self::ProxyGetOwnPropertyDescriptor => "getOwnPropertyDescriptor",
            Self::ProxyDefineProperty => "defineProperty",
            Self::ProxyOwnKeys => "ownKeys",
            Self::Named(name) => name,
        }
    }
}
