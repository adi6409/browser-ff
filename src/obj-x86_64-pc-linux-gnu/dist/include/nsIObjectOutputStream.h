/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIObjectOutputStream.idl
 */

#ifndef __gen_nsIObjectOutputStream_h__
#define __gen_nsIObjectOutputStream_h__


#ifndef __gen_nsIBinaryOutputStream_h__
#include "nsIBinaryOutputStream.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIObjectOutputStream */
#define NS_IOBJECTOUTPUTSTREAM_IID_STR "92c898ac-5fde-4b99-87b3-5d486422094b"

#define NS_IOBJECTOUTPUTSTREAM_IID \
  {0x92c898ac, 0x5fde, 0x4b99, \
    { 0x87, 0xb3, 0x5d, 0x48, 0x64, 0x22, 0x09, 0x4b }}

class NS_NO_VTABLE nsIObjectOutputStream : public nsIBinaryOutputStream {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IOBJECTOUTPUTSTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIObjectOutputStream;

  /* void writeObject (in nsISupports aObject, in boolean aIsStrongRef); */
  NS_IMETHOD WriteObject(nsISupports *aObject, bool aIsStrongRef) = 0;

  /* void writeSingleRefObject (in nsISupports aObject); */
  NS_IMETHOD WriteSingleRefObject(nsISupports *aObject) = 0;

  /* void writeCompoundObject (in nsISupports aObject, in nsIIDRef aIID, in boolean aIsStrongRef); */
  NS_IMETHOD WriteCompoundObject(nsISupports *aObject, const nsIID & aIID, bool aIsStrongRef) = 0;

  /* void writeID (in nsIDRef aID); */
  NS_IMETHOD WriteID(const nsID & aID) = 0;

  /* [notxpcom] charPtr getBuffer (in uint32_t aLength, in uint32_t aAlignMask); */
  NS_IMETHOD_(char *) GetBuffer(uint32_t aLength, uint32_t aAlignMask) = 0;

  /* [notxpcom] void putBuffer (in charPtr aBuffer, in uint32_t aLength); */
  NS_IMETHOD_(void) PutBuffer(char * aBuffer, uint32_t aLength) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIObjectOutputStream, NS_IOBJECTOUTPUTSTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIOBJECTOUTPUTSTREAM \
  NS_IMETHOD WriteObject(nsISupports *aObject, bool aIsStrongRef) override; \
  NS_IMETHOD WriteSingleRefObject(nsISupports *aObject) override; \
  NS_IMETHOD WriteCompoundObject(nsISupports *aObject, const nsIID & aIID, bool aIsStrongRef) override; \
  NS_IMETHOD WriteID(const nsID & aID) override; \
  NS_IMETHOD_(char *) GetBuffer(uint32_t aLength, uint32_t aAlignMask) override; \
  NS_IMETHOD_(void) PutBuffer(char * aBuffer, uint32_t aLength) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIOBJECTOUTPUTSTREAM \
  nsresult WriteObject(nsISupports *aObject, bool aIsStrongRef); \
  nsresult WriteSingleRefObject(nsISupports *aObject); \
  nsresult WriteCompoundObject(nsISupports *aObject, const nsIID & aIID, bool aIsStrongRef); \
  nsresult WriteID(const nsID & aID); \
  nsresult_(char *) GetBuffer(uint32_t aLength, uint32_t aAlignMask); \
  nsresult_(void) PutBuffer(char * aBuffer, uint32_t aLength); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIOBJECTOUTPUTSTREAM(_to) \
  NS_IMETHOD WriteObject(nsISupports *aObject, bool aIsStrongRef) override { return _to WriteObject(aObject, aIsStrongRef); } \
  NS_IMETHOD WriteSingleRefObject(nsISupports *aObject) override { return _to WriteSingleRefObject(aObject); } \
  NS_IMETHOD WriteCompoundObject(nsISupports *aObject, const nsIID & aIID, bool aIsStrongRef) override { return _to WriteCompoundObject(aObject, aIID, aIsStrongRef); } \
  NS_IMETHOD WriteID(const nsID & aID) override { return _to WriteID(aID); } \
  NS_IMETHOD_(char *) GetBuffer(uint32_t aLength, uint32_t aAlignMask) override { return _to GetBuffer(aLength, aAlignMask); } \
  NS_IMETHOD_(void) PutBuffer(char * aBuffer, uint32_t aLength) override { return _to PutBuffer(aBuffer, aLength); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIOBJECTOUTPUTSTREAM(_to) \
  NS_IMETHOD WriteObject(nsISupports *aObject, bool aIsStrongRef) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WriteObject(aObject, aIsStrongRef); } \
  NS_IMETHOD WriteSingleRefObject(nsISupports *aObject) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WriteSingleRefObject(aObject); } \
  NS_IMETHOD WriteCompoundObject(nsISupports *aObject, const nsIID & aIID, bool aIsStrongRef) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WriteCompoundObject(aObject, aIID, aIsStrongRef); } \
  NS_IMETHOD WriteID(const nsID & aID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WriteID(aID); } \
  NS_IMETHOD_(char *) GetBuffer(uint32_t aLength, uint32_t aAlignMask) override; \
  NS_IMETHOD_(void) PutBuffer(char * aBuffer, uint32_t aLength) override; 

already_AddRefed<nsIObjectOutputStream>
NS_NewObjectOutputStream(nsIOutputStream* aOutputStream);
inline nsresult
NS_WriteOptionalObject(nsIObjectOutputStream* aStream, nsISupports* aObject,
                       bool aIsStrongRef)
{
    bool nonnull = (aObject != nullptr);
    nsresult rv = aStream->WriteBoolean(nonnull);
    if (NS_SUCCEEDED(rv) && nonnull)
        rv = aStream->WriteObject(aObject, aIsStrongRef);
    return rv;
}
inline nsresult
NS_WriteOptionalSingleRefObject(nsIObjectOutputStream* aStream,
                                nsISupports* aObject)
{
    bool nonnull = (aObject != nullptr);
    nsresult rv = aStream->WriteBoolean(nonnull);
    if (NS_SUCCEEDED(rv) && nonnull)
        rv = aStream->WriteSingleRefObject(aObject);
    return rv;
}
inline nsresult
NS_WriteOptionalCompoundObject(nsIObjectOutputStream* aStream,
                               nsISupports* aObject,
                               const nsIID& aIID,
                               bool aIsStrongRef)
{
    bool nonnull = (aObject != nullptr);
    nsresult rv = aStream->WriteBoolean(nonnull);
    if (NS_SUCCEEDED(rv) && nonnull)
        rv = aStream->WriteCompoundObject(aObject, aIID, aIsStrongRef);
    return rv;
}

#endif /* __gen_nsIObjectOutputStream_h__ */
