/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIContentViewerEdit.idl
 */

#ifndef __gen_nsIContentViewerEdit_h__
#define __gen_nsIContentViewerEdit_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsINode; /* webidl Node */


/* starting interface:    nsIContentViewerEdit */
#define NS_ICONTENTVIEWEREDIT_IID_STR "35be2d7e-f29b-48ec-bf7e-80a30a724de3"

#define NS_ICONTENTVIEWEREDIT_IID \
  {0x35be2d7e, 0xf29b, 0x48ec, \
    { 0xbf, 0x7e, 0x80, 0xa3, 0x0a, 0x72, 0x4d, 0xe3 }}

class NS_NO_VTABLE nsIContentViewerEdit : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICONTENTVIEWEREDIT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIContentViewerEdit;

  /* void clearSelection (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ClearSelection(void) = 0;

  /* void selectAll (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SelectAll(void) = 0;

  /* void copySelection (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CopySelection(void) = 0;

  /* readonly attribute boolean copyable; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCopyable(bool *aCopyable) = 0;

  /* void copyLinkLocation (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CopyLinkLocation(void) = 0;

  /* readonly attribute boolean inLink; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInLink(bool *aInLink) = 0;

  enum {
    COPY_IMAGE_TEXT = 1,
    COPY_IMAGE_HTML = 2,
    COPY_IMAGE_DATA = 4,
    COPY_IMAGE_ALL = -1
  };

  /* void copyImage (in long aCopyFlags); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CopyImage(int32_t aCopyFlags) = 0;

  /* readonly attribute boolean inImage; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInImage(bool *aInImage) = 0;

  /* AString getContents (in string aMimeType, in boolean aSelectionOnly); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetContents(const char * aMimeType, bool aSelectionOnly, nsAString& _retval) = 0;

  /* readonly attribute boolean canGetContents; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCanGetContents(bool *aCanGetContents) = 0;

  /* void setCommandNode (in Node aNode); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCommandNode(nsINode *aNode) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIContentViewerEdit, NS_ICONTENTVIEWEREDIT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICONTENTVIEWEREDIT \
  NS_IMETHOD ClearSelection(void) override; \
  NS_IMETHOD SelectAll(void) override; \
  NS_IMETHOD CopySelection(void) override; \
  NS_IMETHOD GetCopyable(bool *aCopyable) override; \
  NS_IMETHOD CopyLinkLocation(void) override; \
  NS_IMETHOD GetInLink(bool *aInLink) override; \
  NS_IMETHOD CopyImage(int32_t aCopyFlags) override; \
  NS_IMETHOD GetInImage(bool *aInImage) override; \
  NS_IMETHOD GetContents(const char * aMimeType, bool aSelectionOnly, nsAString& _retval) override; \
  NS_IMETHOD GetCanGetContents(bool *aCanGetContents) override; \
  NS_IMETHOD SetCommandNode(nsINode *aNode) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICONTENTVIEWEREDIT \
  nsresult ClearSelection(void); \
  nsresult SelectAll(void); \
  nsresult CopySelection(void); \
  nsresult GetCopyable(bool *aCopyable); \
  nsresult CopyLinkLocation(void); \
  nsresult GetInLink(bool *aInLink); \
  nsresult CopyImage(int32_t aCopyFlags); \
  nsresult GetInImage(bool *aInImage); \
  nsresult GetContents(const char * aMimeType, bool aSelectionOnly, nsAString& _retval); \
  nsresult GetCanGetContents(bool *aCanGetContents); \
  nsresult SetCommandNode(nsINode *aNode); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICONTENTVIEWEREDIT(_to) \
  NS_IMETHOD ClearSelection(void) override { return _to ClearSelection(); } \
  NS_IMETHOD SelectAll(void) override { return _to SelectAll(); } \
  NS_IMETHOD CopySelection(void) override { return _to CopySelection(); } \
  NS_IMETHOD GetCopyable(bool *aCopyable) override { return _to GetCopyable(aCopyable); } \
  NS_IMETHOD CopyLinkLocation(void) override { return _to CopyLinkLocation(); } \
  NS_IMETHOD GetInLink(bool *aInLink) override { return _to GetInLink(aInLink); } \
  NS_IMETHOD CopyImage(int32_t aCopyFlags) override { return _to CopyImage(aCopyFlags); } \
  NS_IMETHOD GetInImage(bool *aInImage) override { return _to GetInImage(aInImage); } \
  NS_IMETHOD GetContents(const char * aMimeType, bool aSelectionOnly, nsAString& _retval) override { return _to GetContents(aMimeType, aSelectionOnly, _retval); } \
  NS_IMETHOD GetCanGetContents(bool *aCanGetContents) override { return _to GetCanGetContents(aCanGetContents); } \
  NS_IMETHOD SetCommandNode(nsINode *aNode) override { return _to SetCommandNode(aNode); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICONTENTVIEWEREDIT(_to) \
  NS_IMETHOD ClearSelection(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearSelection(); } \
  NS_IMETHOD SelectAll(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SelectAll(); } \
  NS_IMETHOD CopySelection(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CopySelection(); } \
  NS_IMETHOD GetCopyable(bool *aCopyable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCopyable(aCopyable); } \
  NS_IMETHOD CopyLinkLocation(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CopyLinkLocation(); } \
  NS_IMETHOD GetInLink(bool *aInLink) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInLink(aInLink); } \
  NS_IMETHOD CopyImage(int32_t aCopyFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CopyImage(aCopyFlags); } \
  NS_IMETHOD GetInImage(bool *aInImage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInImage(aInImage); } \
  NS_IMETHOD GetContents(const char * aMimeType, bool aSelectionOnly, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContents(aMimeType, aSelectionOnly, _retval); } \
  NS_IMETHOD GetCanGetContents(bool *aCanGetContents) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCanGetContents(aCanGetContents); } \
  NS_IMETHOD SetCommandNode(nsINode *aNode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCommandNode(aNode); } 


#endif /* __gen_nsIContentViewerEdit_h__ */
