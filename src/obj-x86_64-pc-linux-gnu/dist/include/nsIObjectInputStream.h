/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIObjectInputStream.idl
 */

#ifndef __gen_nsIObjectInputStream_h__
#define __gen_nsIObjectInputStream_h__


#ifndef __gen_nsIBinaryInputStream_h__
#include "nsIBinaryInputStream.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIObjectInputStream */
#define NS_IOBJECTINPUTSTREAM_IID_STR "6c248606-4eae-46fa-9df0-ba58502368eb"

#define NS_IOBJECTINPUTSTREAM_IID \
  {0x6c248606, 0x4eae, 0x46fa, \
    { 0x9d, 0xf0, 0xba, 0x58, 0x50, 0x23, 0x68, 0xeb }}

class NS_NO_VTABLE nsIObjectInputStream : public nsIBinaryInputStream {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IOBJECTINPUTSTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIObjectInputStream;

  /* nsISupports readObject (in boolean aIsStrongRef); */
  NS_IMETHOD ReadObject(bool aIsStrongRef, nsISupports **_retval) = 0;

  /* [notxpcom] nsresult readID (out nsID aID); */
  NS_IMETHOD ReadID(nsID * aID) = 0;

  /* [notxpcom] charPtr getBuffer (in uint32_t aLength, in uint32_t aAlignMask); */
  NS_IMETHOD_(char *) GetBuffer(uint32_t aLength, uint32_t aAlignMask) = 0;

  /* [notxpcom] void putBuffer (in charPtr aBuffer, in uint32_t aLength); */
  NS_IMETHOD_(void) PutBuffer(char * aBuffer, uint32_t aLength) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIObjectInputStream, NS_IOBJECTINPUTSTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIOBJECTINPUTSTREAM \
  NS_IMETHOD ReadObject(bool aIsStrongRef, nsISupports **_retval) override; \
  NS_IMETHOD ReadID(nsID * aID) override; \
  NS_IMETHOD_(char *) GetBuffer(uint32_t aLength, uint32_t aAlignMask) override; \
  NS_IMETHOD_(void) PutBuffer(char * aBuffer, uint32_t aLength) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIOBJECTINPUTSTREAM \
  nsresult ReadObject(bool aIsStrongRef, nsISupports **_retval); \
  nsresult ReadID(nsID * aID); \
  nsresult_(char *) GetBuffer(uint32_t aLength, uint32_t aAlignMask); \
  nsresult_(void) PutBuffer(char * aBuffer, uint32_t aLength); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIOBJECTINPUTSTREAM(_to) \
  NS_IMETHOD ReadObject(bool aIsStrongRef, nsISupports **_retval) override { return _to ReadObject(aIsStrongRef, _retval); } \
  NS_IMETHOD ReadID(nsID * aID) override { return _to ReadID(aID); } \
  NS_IMETHOD_(char *) GetBuffer(uint32_t aLength, uint32_t aAlignMask) override { return _to GetBuffer(aLength, aAlignMask); } \
  NS_IMETHOD_(void) PutBuffer(char * aBuffer, uint32_t aLength) override { return _to PutBuffer(aBuffer, aLength); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIOBJECTINPUTSTREAM(_to) \
  NS_IMETHOD ReadObject(bool aIsStrongRef, nsISupports **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReadObject(aIsStrongRef, _retval); } \
  NS_IMETHOD ReadID(nsID * aID) override; \
  NS_IMETHOD_(char *) GetBuffer(uint32_t aLength, uint32_t aAlignMask) override; \
  NS_IMETHOD_(void) PutBuffer(char * aBuffer, uint32_t aLength) override; 


already_AddRefed<nsIObjectInputStream>
NS_NewObjectInputStream(nsIInputStream* aOutputStream);
inline nsresult
NS_ReadOptionalObject(nsIObjectInputStream* aStream, bool aIsStrongRef,
                      nsISupports* *aResult)
{
    bool nonnull;
    nsresult rv = aStream->ReadBoolean(&nonnull);
    if (NS_SUCCEEDED(rv)) {
        if (nonnull)
            rv = aStream->ReadObject(aIsStrongRef, aResult);
        else
            *aResult = nullptr;
    }
    return rv;
}

#endif /* __gen_nsIObjectInputStream_h__ */
