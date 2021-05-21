/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIVariant.idl
 */

#ifndef __gen_nsIVariant_h__
#define __gen_nsIVariant_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include "xptinfo.h"
// This enum class used to be a const-only XPIDL interface, containing literal
// integer descriptions of the different fields. Instead, it now directly
// references the nsXPTTypeTag variants VTYPE_ are intended to match.
struct nsIDataType
{
    enum {
        // These MUST match the declarations in xptinfo.h.
        // Otherwise the world is likely to explode.
        VTYPE_INT8              = TD_INT8             ,
        VTYPE_INT16             = TD_INT16            ,
        VTYPE_INT32             = TD_INT32            ,
        VTYPE_INT64             = TD_INT64            ,
        VTYPE_UINT8             = TD_UINT8            ,
        VTYPE_UINT16            = TD_UINT16           ,
        VTYPE_UINT32            = TD_UINT32           ,
        VTYPE_UINT64            = TD_UINT64           ,
        VTYPE_FLOAT             = TD_FLOAT            ,
        VTYPE_DOUBLE            = TD_DOUBLE           ,
        VTYPE_BOOL              = TD_BOOL             ,
        VTYPE_CHAR              = TD_CHAR             ,
        VTYPE_WCHAR             = TD_WCHAR            ,
        VTYPE_VOID              = TD_VOID             ,
        VTYPE_ID                = TD_NSIDPTR          ,
        VTYPE_CHAR_STR          = TD_PSTRING          ,
        VTYPE_WCHAR_STR         = TD_PWSTRING         ,
        VTYPE_INTERFACE         = TD_INTERFACE_TYPE   ,
        VTYPE_INTERFACE_IS      = TD_INTERFACE_IS_TYPE,
        VTYPE_ARRAY             = TD_LEGACY_ARRAY     ,
        VTYPE_STRING_SIZE_IS    = TD_PSTRING_SIZE_IS  ,
        VTYPE_WSTRING_SIZE_IS   = TD_PWSTRING_SIZE_IS ,
        VTYPE_UTF8STRING        = TD_UTF8STRING       ,
        VTYPE_CSTRING           = TD_CSTRING          ,
        VTYPE_ASTRING           = TD_ASTRING          ,
        // Non-xpt variant types
        VTYPE_EMPTY_ARRAY       = 254                 ,
        VTYPE_EMPTY             = 255
    };
};

/* starting interface:    nsIVariant */
#define NS_IVARIANT_IID_STR "81e4c2de-acac-4ad6-901a-b5fb1b851a0d"

#define NS_IVARIANT_IID \
  {0x81e4c2de, 0xacac, 0x4ad6, \
    { 0x90, 0x1a, 0xb5, 0xfb, 0x1b, 0x85, 0x1a, 0x0d }}

class NS_NO_VTABLE nsIVariant : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IVARIANT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIVariant;

  /* [nostdcall,notxpcom] readonly attribute uint16_t dataType; */
  virtual uint16_t GetDataType() = 0;

  /* [noscript] uint8_t getAsInt8 (); */
  NS_IMETHOD GetAsInt8(uint8_t *_retval) = 0;

  /* [noscript] int16_t getAsInt16 (); */
  NS_IMETHOD GetAsInt16(int16_t *_retval) = 0;

  /* [noscript] int32_t getAsInt32 (); */
  NS_IMETHOD GetAsInt32(int32_t *_retval) = 0;

  /* [noscript] int64_t getAsInt64 (); */
  NS_IMETHOD GetAsInt64(int64_t *_retval) = 0;

  /* [noscript] uint8_t getAsUint8 (); */
  NS_IMETHOD GetAsUint8(uint8_t *_retval) = 0;

  /* [noscript] uint16_t getAsUint16 (); */
  NS_IMETHOD GetAsUint16(uint16_t *_retval) = 0;

  /* [noscript] uint32_t getAsUint32 (); */
  NS_IMETHOD GetAsUint32(uint32_t *_retval) = 0;

  /* [noscript] uint64_t getAsUint64 (); */
  NS_IMETHOD GetAsUint64(uint64_t *_retval) = 0;

  /* [noscript] float getAsFloat (); */
  NS_IMETHOD GetAsFloat(float *_retval) = 0;

  /* [noscript] double getAsDouble (); */
  NS_IMETHOD GetAsDouble(double *_retval) = 0;

  /* [noscript] boolean getAsBool (); */
  NS_IMETHOD GetAsBool(bool *_retval) = 0;

  /* [noscript] char getAsChar (); */
  NS_IMETHOD GetAsChar(char *_retval) = 0;

  /* [noscript] wchar getAsWChar (); */
  NS_IMETHOD GetAsWChar(char16_t *_retval) = 0;

  /* [notxpcom] nsresult getAsID (out nsID retval); */
  NS_IMETHOD GetAsID(nsID * retval) = 0;

  /* [noscript] AString getAsAString (); */
  NS_IMETHOD GetAsAString(nsAString& _retval) = 0;

  /* [noscript] ACString getAsACString (); */
  NS_IMETHOD GetAsACString(nsACString& _retval) = 0;

  /* [noscript] AUTF8String getAsAUTF8String (); */
  NS_IMETHOD GetAsAUTF8String(nsACString& _retval) = 0;

  /* [noscript] string getAsString (); */
  NS_IMETHOD GetAsString(char * *_retval) = 0;

  /* [noscript] wstring getAsWString (); */
  NS_IMETHOD GetAsWString(char16_t * *_retval) = 0;

  /* [noscript] nsISupports getAsISupports (); */
  NS_IMETHOD GetAsISupports(nsISupports **_retval) = 0;

  /* [noscript] jsval getAsJSVal (); */
  NS_IMETHOD GetAsJSVal(JS::MutableHandleValue _retval) = 0;

  /* [noscript] void getAsInterface (out nsIIDPtr iid, [iid_is (iid), retval] out nsQIResult iface); */
  NS_IMETHOD GetAsInterface(nsIID * * iid, void * * iface) = 0;

  /* [notxpcom] nsresult getAsArray (out uint16_t type, out nsIID iid, out uint32_t count, out voidPtr ptr); */
  NS_IMETHOD GetAsArray(uint16_t *type, nsIID * iid, uint32_t *count, void * * ptr) = 0;

  /* [noscript] void getAsStringWithSize (out uint32_t size, [size_is (size), retval] out string str); */
  NS_IMETHOD GetAsStringWithSize(uint32_t *size, char * *str) = 0;

  /* [noscript] void getAsWStringWithSize (out uint32_t size, [size_is (size), retval] out wstring str); */
  NS_IMETHOD GetAsWStringWithSize(uint32_t *size, char16_t * *str) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIVariant, NS_IVARIANT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIVARIANT \
  virtual uint16_t GetDataType() override; \
  NS_IMETHOD GetAsInt8(uint8_t *_retval) override; \
  NS_IMETHOD GetAsInt16(int16_t *_retval) override; \
  NS_IMETHOD GetAsInt32(int32_t *_retval) override; \
  NS_IMETHOD GetAsInt64(int64_t *_retval) override; \
  NS_IMETHOD GetAsUint8(uint8_t *_retval) override; \
  NS_IMETHOD GetAsUint16(uint16_t *_retval) override; \
  NS_IMETHOD GetAsUint32(uint32_t *_retval) override; \
  NS_IMETHOD GetAsUint64(uint64_t *_retval) override; \
  NS_IMETHOD GetAsFloat(float *_retval) override; \
  NS_IMETHOD GetAsDouble(double *_retval) override; \
  NS_IMETHOD GetAsBool(bool *_retval) override; \
  NS_IMETHOD GetAsChar(char *_retval) override; \
  NS_IMETHOD GetAsWChar(char16_t *_retval) override; \
  NS_IMETHOD GetAsID(nsID * retval) override; \
  NS_IMETHOD GetAsAString(nsAString& _retval) override; \
  NS_IMETHOD GetAsACString(nsACString& _retval) override; \
  NS_IMETHOD GetAsAUTF8String(nsACString& _retval) override; \
  NS_IMETHOD GetAsString(char * *_retval) override; \
  NS_IMETHOD GetAsWString(char16_t * *_retval) override; \
  NS_IMETHOD GetAsISupports(nsISupports **_retval) override; \
  NS_IMETHOD GetAsJSVal(JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetAsInterface(nsIID * * iid, void * * iface) override; \
  NS_IMETHOD GetAsArray(uint16_t *type, nsIID * iid, uint32_t *count, void * * ptr) override; \
  NS_IMETHOD GetAsStringWithSize(uint32_t *size, char * *str) override; \
  NS_IMETHOD GetAsWStringWithSize(uint32_t *size, char16_t * *str) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIVARIANT \
  uint16_t GetDataType(); \
  nsresult GetAsInt8(uint8_t *_retval); \
  nsresult GetAsInt16(int16_t *_retval); \
  nsresult GetAsInt32(int32_t *_retval); \
  nsresult GetAsInt64(int64_t *_retval); \
  nsresult GetAsUint8(uint8_t *_retval); \
  nsresult GetAsUint16(uint16_t *_retval); \
  nsresult GetAsUint32(uint32_t *_retval); \
  nsresult GetAsUint64(uint64_t *_retval); \
  nsresult GetAsFloat(float *_retval); \
  nsresult GetAsDouble(double *_retval); \
  nsresult GetAsBool(bool *_retval); \
  nsresult GetAsChar(char *_retval); \
  nsresult GetAsWChar(char16_t *_retval); \
  nsresult GetAsID(nsID * retval); \
  nsresult GetAsAString(nsAString& _retval); \
  nsresult GetAsACString(nsACString& _retval); \
  nsresult GetAsAUTF8String(nsACString& _retval); \
  nsresult GetAsString(char * *_retval); \
  nsresult GetAsWString(char16_t * *_retval); \
  nsresult GetAsISupports(nsISupports **_retval); \
  nsresult GetAsJSVal(JS::MutableHandleValue _retval); \
  nsresult GetAsInterface(nsIID * * iid, void * * iface); \
  nsresult GetAsArray(uint16_t *type, nsIID * iid, uint32_t *count, void * * ptr); \
  nsresult GetAsStringWithSize(uint32_t *size, char * *str); \
  nsresult GetAsWStringWithSize(uint32_t *size, char16_t * *str); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIVARIANT(_to) \
  virtual uint16_t GetDataType() override { return _to GetDataType(); } \
  NS_IMETHOD GetAsInt8(uint8_t *_retval) override { return _to GetAsInt8(_retval); } \
  NS_IMETHOD GetAsInt16(int16_t *_retval) override { return _to GetAsInt16(_retval); } \
  NS_IMETHOD GetAsInt32(int32_t *_retval) override { return _to GetAsInt32(_retval); } \
  NS_IMETHOD GetAsInt64(int64_t *_retval) override { return _to GetAsInt64(_retval); } \
  NS_IMETHOD GetAsUint8(uint8_t *_retval) override { return _to GetAsUint8(_retval); } \
  NS_IMETHOD GetAsUint16(uint16_t *_retval) override { return _to GetAsUint16(_retval); } \
  NS_IMETHOD GetAsUint32(uint32_t *_retval) override { return _to GetAsUint32(_retval); } \
  NS_IMETHOD GetAsUint64(uint64_t *_retval) override { return _to GetAsUint64(_retval); } \
  NS_IMETHOD GetAsFloat(float *_retval) override { return _to GetAsFloat(_retval); } \
  NS_IMETHOD GetAsDouble(double *_retval) override { return _to GetAsDouble(_retval); } \
  NS_IMETHOD GetAsBool(bool *_retval) override { return _to GetAsBool(_retval); } \
  NS_IMETHOD GetAsChar(char *_retval) override { return _to GetAsChar(_retval); } \
  NS_IMETHOD GetAsWChar(char16_t *_retval) override { return _to GetAsWChar(_retval); } \
  NS_IMETHOD GetAsID(nsID * retval) override { return _to GetAsID(retval); } \
  NS_IMETHOD GetAsAString(nsAString& _retval) override { return _to GetAsAString(_retval); } \
  NS_IMETHOD GetAsACString(nsACString& _retval) override { return _to GetAsACString(_retval); } \
  NS_IMETHOD GetAsAUTF8String(nsACString& _retval) override { return _to GetAsAUTF8String(_retval); } \
  NS_IMETHOD GetAsString(char * *_retval) override { return _to GetAsString(_retval); } \
  NS_IMETHOD GetAsWString(char16_t * *_retval) override { return _to GetAsWString(_retval); } \
  NS_IMETHOD GetAsISupports(nsISupports **_retval) override { return _to GetAsISupports(_retval); } \
  NS_IMETHOD GetAsJSVal(JS::MutableHandleValue _retval) override { return _to GetAsJSVal(_retval); } \
  NS_IMETHOD GetAsInterface(nsIID * * iid, void * * iface) override { return _to GetAsInterface(iid, iface); } \
  NS_IMETHOD GetAsArray(uint16_t *type, nsIID * iid, uint32_t *count, void * * ptr) override { return _to GetAsArray(type, iid, count, ptr); } \
  NS_IMETHOD GetAsStringWithSize(uint32_t *size, char * *str) override { return _to GetAsStringWithSize(size, str); } \
  NS_IMETHOD GetAsWStringWithSize(uint32_t *size, char16_t * *str) override { return _to GetAsWStringWithSize(size, str); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIVARIANT(_to) \
  virtual uint16_t GetDataType() override; \
  NS_IMETHOD GetAsInt8(uint8_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsInt8(_retval); } \
  NS_IMETHOD GetAsInt16(int16_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsInt16(_retval); } \
  NS_IMETHOD GetAsInt32(int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsInt32(_retval); } \
  NS_IMETHOD GetAsInt64(int64_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsInt64(_retval); } \
  NS_IMETHOD GetAsUint8(uint8_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsUint8(_retval); } \
  NS_IMETHOD GetAsUint16(uint16_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsUint16(_retval); } \
  NS_IMETHOD GetAsUint32(uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsUint32(_retval); } \
  NS_IMETHOD GetAsUint64(uint64_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsUint64(_retval); } \
  NS_IMETHOD GetAsFloat(float *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsFloat(_retval); } \
  NS_IMETHOD GetAsDouble(double *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsDouble(_retval); } \
  NS_IMETHOD GetAsBool(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsBool(_retval); } \
  NS_IMETHOD GetAsChar(char *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsChar(_retval); } \
  NS_IMETHOD GetAsWChar(char16_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsWChar(_retval); } \
  NS_IMETHOD GetAsID(nsID * retval) override; \
  NS_IMETHOD GetAsAString(nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsAString(_retval); } \
  NS_IMETHOD GetAsACString(nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsACString(_retval); } \
  NS_IMETHOD GetAsAUTF8String(nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsAUTF8String(_retval); } \
  NS_IMETHOD GetAsString(char * *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsString(_retval); } \
  NS_IMETHOD GetAsWString(char16_t * *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsWString(_retval); } \
  NS_IMETHOD GetAsISupports(nsISupports **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsISupports(_retval); } \
  NS_IMETHOD GetAsJSVal(JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsJSVal(_retval); } \
  NS_IMETHOD GetAsInterface(nsIID * * iid, void * * iface) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsInterface(iid, iface); } \
  NS_IMETHOD GetAsArray(uint16_t *type, nsIID * iid, uint32_t *count, void * * ptr) override; \
  NS_IMETHOD GetAsStringWithSize(uint32_t *size, char * *str) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsStringWithSize(size, str); } \
  NS_IMETHOD GetAsWStringWithSize(uint32_t *size, char16_t * *str) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsWStringWithSize(size, str); } 


/* starting interface:    nsIWritableVariant */
#define NS_IWRITABLEVARIANT_IID_STR "5586a590-8c82-11d5-90f3-0010a4e73d9a"

#define NS_IWRITABLEVARIANT_IID \
  {0x5586a590, 0x8c82, 0x11d5, \
    { 0x90, 0xf3, 0x00, 0x10, 0xa4, 0xe7, 0x3d, 0x9a }}

class NS_NO_VTABLE nsIWritableVariant : public nsIVariant {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWRITABLEVARIANT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWritableVariant;

  /* attribute boolean writable; */
  NS_IMETHOD GetWritable(bool *aWritable) = 0;
  NS_IMETHOD SetWritable(bool aWritable) = 0;

  /* void setAsInt8 (in uint8_t aValue); */
  NS_IMETHOD SetAsInt8(uint8_t aValue) = 0;

  /* void setAsInt16 (in int16_t aValue); */
  NS_IMETHOD SetAsInt16(int16_t aValue) = 0;

  /* void setAsInt32 (in int32_t aValue); */
  NS_IMETHOD SetAsInt32(int32_t aValue) = 0;

  /* void setAsInt64 (in int64_t aValue); */
  NS_IMETHOD SetAsInt64(int64_t aValue) = 0;

  /* void setAsUint8 (in uint8_t aValue); */
  NS_IMETHOD SetAsUint8(uint8_t aValue) = 0;

  /* void setAsUint16 (in uint16_t aValue); */
  NS_IMETHOD SetAsUint16(uint16_t aValue) = 0;

  /* void setAsUint32 (in uint32_t aValue); */
  NS_IMETHOD SetAsUint32(uint32_t aValue) = 0;

  /* void setAsUint64 (in uint64_t aValue); */
  NS_IMETHOD SetAsUint64(uint64_t aValue) = 0;

  /* void setAsFloat (in float aValue); */
  NS_IMETHOD SetAsFloat(float aValue) = 0;

  /* void setAsDouble (in double aValue); */
  NS_IMETHOD SetAsDouble(double aValue) = 0;

  /* void setAsBool (in boolean aValue); */
  NS_IMETHOD SetAsBool(bool aValue) = 0;

  /* void setAsChar (in char aValue); */
  NS_IMETHOD SetAsChar(char aValue) = 0;

  /* void setAsWChar (in wchar aValue); */
  NS_IMETHOD SetAsWChar(char16_t aValue) = 0;

  /* void setAsID (in nsIDRef aValue); */
  NS_IMETHOD SetAsID(const nsID & aValue) = 0;

  /* void setAsAString (in AString aValue); */
  NS_IMETHOD SetAsAString(const nsAString& aValue) = 0;

  /* void setAsACString (in ACString aValue); */
  NS_IMETHOD SetAsACString(const nsACString& aValue) = 0;

  /* void setAsAUTF8String (in AUTF8String aValue); */
  NS_IMETHOD SetAsAUTF8String(const nsACString& aValue) = 0;

  /* void setAsString (in string aValue); */
  NS_IMETHOD SetAsString(const char * aValue) = 0;

  /* void setAsWString (in wstring aValue); */
  NS_IMETHOD SetAsWString(const char16_t * aValue) = 0;

  /* void setAsISupports (in nsISupports aValue); */
  NS_IMETHOD SetAsISupports(nsISupports *aValue) = 0;

  /* void setAsInterface (in nsIIDRef iid, [iid_is (iid)] in nsQIResult iface); */
  NS_IMETHOD SetAsInterface(const nsIID & iid, void * iface) = 0;

  /* [noscript] void setAsArray (in uint16_t type, in nsIIDPtr iid, in uint32_t count, in voidPtr ptr); */
  NS_IMETHOD SetAsArray(uint16_t type, const nsIID * iid, uint32_t count, void * ptr) = 0;

  /* void setAsStringWithSize (in uint32_t size, [size_is (size)] in string str); */
  NS_IMETHOD SetAsStringWithSize(uint32_t size, const char * str) = 0;

  /* void setAsWStringWithSize (in uint32_t size, [size_is (size)] in wstring str); */
  NS_IMETHOD SetAsWStringWithSize(uint32_t size, const char16_t * str) = 0;

  /* void setAsVoid (); */
  NS_IMETHOD SetAsVoid(void) = 0;

  /* void setAsEmpty (); */
  NS_IMETHOD SetAsEmpty(void) = 0;

  /* void setAsEmptyArray (); */
  NS_IMETHOD SetAsEmptyArray(void) = 0;

  /* void setFromVariant (in nsIVariant aValue); */
  NS_IMETHOD SetFromVariant(nsIVariant *aValue) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWritableVariant, NS_IWRITABLEVARIANT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWRITABLEVARIANT \
  NS_IMETHOD GetWritable(bool *aWritable) override; \
  NS_IMETHOD SetWritable(bool aWritable) override; \
  NS_IMETHOD SetAsInt8(uint8_t aValue) override; \
  NS_IMETHOD SetAsInt16(int16_t aValue) override; \
  NS_IMETHOD SetAsInt32(int32_t aValue) override; \
  NS_IMETHOD SetAsInt64(int64_t aValue) override; \
  NS_IMETHOD SetAsUint8(uint8_t aValue) override; \
  NS_IMETHOD SetAsUint16(uint16_t aValue) override; \
  NS_IMETHOD SetAsUint32(uint32_t aValue) override; \
  NS_IMETHOD SetAsUint64(uint64_t aValue) override; \
  NS_IMETHOD SetAsFloat(float aValue) override; \
  NS_IMETHOD SetAsDouble(double aValue) override; \
  NS_IMETHOD SetAsBool(bool aValue) override; \
  NS_IMETHOD SetAsChar(char aValue) override; \
  NS_IMETHOD SetAsWChar(char16_t aValue) override; \
  NS_IMETHOD SetAsID(const nsID & aValue) override; \
  NS_IMETHOD SetAsAString(const nsAString& aValue) override; \
  NS_IMETHOD SetAsACString(const nsACString& aValue) override; \
  NS_IMETHOD SetAsAUTF8String(const nsACString& aValue) override; \
  NS_IMETHOD SetAsString(const char * aValue) override; \
  NS_IMETHOD SetAsWString(const char16_t * aValue) override; \
  NS_IMETHOD SetAsISupports(nsISupports *aValue) override; \
  NS_IMETHOD SetAsInterface(const nsIID & iid, void * iface) override; \
  NS_IMETHOD SetAsArray(uint16_t type, const nsIID * iid, uint32_t count, void * ptr) override; \
  NS_IMETHOD SetAsStringWithSize(uint32_t size, const char * str) override; \
  NS_IMETHOD SetAsWStringWithSize(uint32_t size, const char16_t * str) override; \
  NS_IMETHOD SetAsVoid(void) override; \
  NS_IMETHOD SetAsEmpty(void) override; \
  NS_IMETHOD SetAsEmptyArray(void) override; \
  NS_IMETHOD SetFromVariant(nsIVariant *aValue) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWRITABLEVARIANT \
  nsresult GetWritable(bool *aWritable); \
  nsresult SetWritable(bool aWritable); \
  nsresult SetAsInt8(uint8_t aValue); \
  nsresult SetAsInt16(int16_t aValue); \
  nsresult SetAsInt32(int32_t aValue); \
  nsresult SetAsInt64(int64_t aValue); \
  nsresult SetAsUint8(uint8_t aValue); \
  nsresult SetAsUint16(uint16_t aValue); \
  nsresult SetAsUint32(uint32_t aValue); \
  nsresult SetAsUint64(uint64_t aValue); \
  nsresult SetAsFloat(float aValue); \
  nsresult SetAsDouble(double aValue); \
  nsresult SetAsBool(bool aValue); \
  nsresult SetAsChar(char aValue); \
  nsresult SetAsWChar(char16_t aValue); \
  nsresult SetAsID(const nsID & aValue); \
  nsresult SetAsAString(const nsAString& aValue); \
  nsresult SetAsACString(const nsACString& aValue); \
  nsresult SetAsAUTF8String(const nsACString& aValue); \
  nsresult SetAsString(const char * aValue); \
  nsresult SetAsWString(const char16_t * aValue); \
  nsresult SetAsISupports(nsISupports *aValue); \
  nsresult SetAsInterface(const nsIID & iid, void * iface); \
  nsresult SetAsArray(uint16_t type, const nsIID * iid, uint32_t count, void * ptr); \
  nsresult SetAsStringWithSize(uint32_t size, const char * str); \
  nsresult SetAsWStringWithSize(uint32_t size, const char16_t * str); \
  nsresult SetAsVoid(void); \
  nsresult SetAsEmpty(void); \
  nsresult SetAsEmptyArray(void); \
  nsresult SetFromVariant(nsIVariant *aValue); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWRITABLEVARIANT(_to) \
  NS_IMETHOD GetWritable(bool *aWritable) override { return _to GetWritable(aWritable); } \
  NS_IMETHOD SetWritable(bool aWritable) override { return _to SetWritable(aWritable); } \
  NS_IMETHOD SetAsInt8(uint8_t aValue) override { return _to SetAsInt8(aValue); } \
  NS_IMETHOD SetAsInt16(int16_t aValue) override { return _to SetAsInt16(aValue); } \
  NS_IMETHOD SetAsInt32(int32_t aValue) override { return _to SetAsInt32(aValue); } \
  NS_IMETHOD SetAsInt64(int64_t aValue) override { return _to SetAsInt64(aValue); } \
  NS_IMETHOD SetAsUint8(uint8_t aValue) override { return _to SetAsUint8(aValue); } \
  NS_IMETHOD SetAsUint16(uint16_t aValue) override { return _to SetAsUint16(aValue); } \
  NS_IMETHOD SetAsUint32(uint32_t aValue) override { return _to SetAsUint32(aValue); } \
  NS_IMETHOD SetAsUint64(uint64_t aValue) override { return _to SetAsUint64(aValue); } \
  NS_IMETHOD SetAsFloat(float aValue) override { return _to SetAsFloat(aValue); } \
  NS_IMETHOD SetAsDouble(double aValue) override { return _to SetAsDouble(aValue); } \
  NS_IMETHOD SetAsBool(bool aValue) override { return _to SetAsBool(aValue); } \
  NS_IMETHOD SetAsChar(char aValue) override { return _to SetAsChar(aValue); } \
  NS_IMETHOD SetAsWChar(char16_t aValue) override { return _to SetAsWChar(aValue); } \
  NS_IMETHOD SetAsID(const nsID & aValue) override { return _to SetAsID(aValue); } \
  NS_IMETHOD SetAsAString(const nsAString& aValue) override { return _to SetAsAString(aValue); } \
  NS_IMETHOD SetAsACString(const nsACString& aValue) override { return _to SetAsACString(aValue); } \
  NS_IMETHOD SetAsAUTF8String(const nsACString& aValue) override { return _to SetAsAUTF8String(aValue); } \
  NS_IMETHOD SetAsString(const char * aValue) override { return _to SetAsString(aValue); } \
  NS_IMETHOD SetAsWString(const char16_t * aValue) override { return _to SetAsWString(aValue); } \
  NS_IMETHOD SetAsISupports(nsISupports *aValue) override { return _to SetAsISupports(aValue); } \
  NS_IMETHOD SetAsInterface(const nsIID & iid, void * iface) override { return _to SetAsInterface(iid, iface); } \
  NS_IMETHOD SetAsArray(uint16_t type, const nsIID * iid, uint32_t count, void * ptr) override { return _to SetAsArray(type, iid, count, ptr); } \
  NS_IMETHOD SetAsStringWithSize(uint32_t size, const char * str) override { return _to SetAsStringWithSize(size, str); } \
  NS_IMETHOD SetAsWStringWithSize(uint32_t size, const char16_t * str) override { return _to SetAsWStringWithSize(size, str); } \
  NS_IMETHOD SetAsVoid(void) override { return _to SetAsVoid(); } \
  NS_IMETHOD SetAsEmpty(void) override { return _to SetAsEmpty(); } \
  NS_IMETHOD SetAsEmptyArray(void) override { return _to SetAsEmptyArray(); } \
  NS_IMETHOD SetFromVariant(nsIVariant *aValue) override { return _to SetFromVariant(aValue); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWRITABLEVARIANT(_to) \
  NS_IMETHOD GetWritable(bool *aWritable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWritable(aWritable); } \
  NS_IMETHOD SetWritable(bool aWritable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetWritable(aWritable); } \
  NS_IMETHOD SetAsInt8(uint8_t aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsInt8(aValue); } \
  NS_IMETHOD SetAsInt16(int16_t aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsInt16(aValue); } \
  NS_IMETHOD SetAsInt32(int32_t aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsInt32(aValue); } \
  NS_IMETHOD SetAsInt64(int64_t aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsInt64(aValue); } \
  NS_IMETHOD SetAsUint8(uint8_t aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsUint8(aValue); } \
  NS_IMETHOD SetAsUint16(uint16_t aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsUint16(aValue); } \
  NS_IMETHOD SetAsUint32(uint32_t aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsUint32(aValue); } \
  NS_IMETHOD SetAsUint64(uint64_t aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsUint64(aValue); } \
  NS_IMETHOD SetAsFloat(float aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsFloat(aValue); } \
  NS_IMETHOD SetAsDouble(double aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsDouble(aValue); } \
  NS_IMETHOD SetAsBool(bool aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsBool(aValue); } \
  NS_IMETHOD SetAsChar(char aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsChar(aValue); } \
  NS_IMETHOD SetAsWChar(char16_t aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsWChar(aValue); } \
  NS_IMETHOD SetAsID(const nsID & aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsID(aValue); } \
  NS_IMETHOD SetAsAString(const nsAString& aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsAString(aValue); } \
  NS_IMETHOD SetAsACString(const nsACString& aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsACString(aValue); } \
  NS_IMETHOD SetAsAUTF8String(const nsACString& aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsAUTF8String(aValue); } \
  NS_IMETHOD SetAsString(const char * aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsString(aValue); } \
  NS_IMETHOD SetAsWString(const char16_t * aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsWString(aValue); } \
  NS_IMETHOD SetAsISupports(nsISupports *aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsISupports(aValue); } \
  NS_IMETHOD SetAsInterface(const nsIID & iid, void * iface) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsInterface(iid, iface); } \
  NS_IMETHOD SetAsArray(uint16_t type, const nsIID * iid, uint32_t count, void * ptr) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsArray(type, iid, count, ptr); } \
  NS_IMETHOD SetAsStringWithSize(uint32_t size, const char * str) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsStringWithSize(size, str); } \
  NS_IMETHOD SetAsWStringWithSize(uint32_t size, const char16_t * str) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsWStringWithSize(size, str); } \
  NS_IMETHOD SetAsVoid(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsVoid(); } \
  NS_IMETHOD SetAsEmpty(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsEmpty(); } \
  NS_IMETHOD SetAsEmptyArray(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsEmptyArray(); } \
  NS_IMETHOD SetFromVariant(nsIVariant *aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFromVariant(aValue); } 

// The contractID for the generic implementation built in to xpcom.
#define NS_VARIANT_CONTRACTID "@mozilla.org/variant;1"

#endif /* __gen_nsIVariant_h__ */
