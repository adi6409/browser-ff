/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/nsIDOMNavigatorUserMedia.idl
 */

#ifndef __gen_nsIDOMNavigatorUserMedia_h__
#define __gen_nsIDOMNavigatorUserMedia_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIVariant_h__
#include "nsIVariant.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIMediaDevice */
#define NS_IMEDIADEVICE_IID_STR "ba3b2e08-1c07-4cd3-8822-f4d7e35ff2ae"

#define NS_IMEDIADEVICE_IID \
  {0xba3b2e08, 0x1c07, 0x4cd3, \
    { 0x88, 0x22, 0xf4, 0xd7, 0xe3, 0x5f, 0xf2, 0xae }}

class NS_NO_VTABLE nsIMediaDevice : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IMEDIADEVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIMediaDevice;

  /* readonly attribute AString type; */
  NS_IMETHOD GetType(nsAString& aType) = 0;

  /* readonly attribute AString name; */
  NS_IMETHOD GetName(nsAString& aName) = 0;

  /* readonly attribute AString id; */
  NS_IMETHOD GetId(nsAString& aId) = 0;

  /* readonly attribute AString mediaSource; */
  NS_IMETHOD GetMediaSource(nsAString& aMediaSource) = 0;

  /* readonly attribute AString rawId; */
  NS_IMETHOD GetRawId(nsAString& aRawId) = 0;

  /* readonly attribute AString groupId; */
  NS_IMETHOD GetGroupId(nsAString& aGroupId) = 0;

  /* readonly attribute AString rawGroupId; */
  NS_IMETHOD GetRawGroupId(nsAString& aRawGroupId) = 0;

  /* readonly attribute boolean scary; */
  NS_IMETHOD GetScary(bool *aScary) = 0;

  /* readonly attribute AString rawName; */
  NS_IMETHOD GetRawName(nsAString& aRawName) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIMediaDevice, NS_IMEDIADEVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIMEDIADEVICE \
  NS_IMETHOD GetType(nsAString& aType) override; \
  NS_IMETHOD GetName(nsAString& aName) override; \
  NS_IMETHOD GetId(nsAString& aId) override; \
  NS_IMETHOD GetMediaSource(nsAString& aMediaSource) override; \
  NS_IMETHOD GetRawId(nsAString& aRawId) override; \
  NS_IMETHOD GetGroupId(nsAString& aGroupId) override; \
  NS_IMETHOD GetRawGroupId(nsAString& aRawGroupId) override; \
  NS_IMETHOD GetScary(bool *aScary) override; \
  NS_IMETHOD GetRawName(nsAString& aRawName) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIMEDIADEVICE \
  nsresult GetType(nsAString& aType); \
  nsresult GetName(nsAString& aName); \
  nsresult GetId(nsAString& aId); \
  nsresult GetMediaSource(nsAString& aMediaSource); \
  nsresult GetRawId(nsAString& aRawId); \
  nsresult GetGroupId(nsAString& aGroupId); \
  nsresult GetRawGroupId(nsAString& aRawGroupId); \
  nsresult GetScary(bool *aScary); \
  nsresult GetRawName(nsAString& aRawName); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIMEDIADEVICE(_to) \
  NS_IMETHOD GetType(nsAString& aType) override { return _to GetType(aType); } \
  NS_IMETHOD GetName(nsAString& aName) override { return _to GetName(aName); } \
  NS_IMETHOD GetId(nsAString& aId) override { return _to GetId(aId); } \
  NS_IMETHOD GetMediaSource(nsAString& aMediaSource) override { return _to GetMediaSource(aMediaSource); } \
  NS_IMETHOD GetRawId(nsAString& aRawId) override { return _to GetRawId(aRawId); } \
  NS_IMETHOD GetGroupId(nsAString& aGroupId) override { return _to GetGroupId(aGroupId); } \
  NS_IMETHOD GetRawGroupId(nsAString& aRawGroupId) override { return _to GetRawGroupId(aRawGroupId); } \
  NS_IMETHOD GetScary(bool *aScary) override { return _to GetScary(aScary); } \
  NS_IMETHOD GetRawName(nsAString& aRawName) override { return _to GetRawName(aRawName); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIMEDIADEVICE(_to) \
  NS_IMETHOD GetType(nsAString& aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetType(aType); } \
  NS_IMETHOD GetName(nsAString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetName(aName); } \
  NS_IMETHOD GetId(nsAString& aId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetId(aId); } \
  NS_IMETHOD GetMediaSource(nsAString& aMediaSource) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMediaSource(aMediaSource); } \
  NS_IMETHOD GetRawId(nsAString& aRawId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRawId(aRawId); } \
  NS_IMETHOD GetGroupId(nsAString& aGroupId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetGroupId(aGroupId); } \
  NS_IMETHOD GetRawGroupId(nsAString& aRawGroupId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRawGroupId(aRawGroupId); } \
  NS_IMETHOD GetScary(bool *aScary) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScary(aScary); } \
  NS_IMETHOD GetRawName(nsAString& aRawName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRawName(aRawName); } 


#endif /* __gen_nsIDOMNavigatorUserMedia_h__ */
