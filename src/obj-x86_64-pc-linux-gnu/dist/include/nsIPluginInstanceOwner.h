/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/plugins/base/nsIPluginInstanceOwner.idl
 */

#ifndef __gen_nsIPluginInstanceOwner_h__
#define __gen_nsIPluginInstanceOwner_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nspluginroot_h__
#include "nspluginroot.h"
#endif

#ifndef __gen_nsIInputStream_h__
#include "nsIInputStream.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
namespace dom {
class Document; /* webidl Document */
} // namespace dom
} // namespace mozilla

#include "npapi.h"
#include "mozilla/EventForwards.h"
class nsNPAPIPluginInstance;
enum nsPluginTagType {
  nsPluginTagType_Unknown,
  nsPluginTagType_Embed,
  nsPluginTagType_Object
};

/* starting interface:    nsIPluginInstanceOwner */
#define NS_IPLUGININSTANCEOWNER_IID_STR "7d65452e-c167-4cba-a0e3-ddc61bdde8c3"

#define NS_IPLUGININSTANCEOWNER_IID \
  {0x7d65452e, 0xc167, 0x4cba, \
    { 0xa0, 0xe3, 0xdd, 0xc6, 0x1b, 0xdd, 0xe8, 0xc3 }}

class nsIPluginInstanceOwner : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPLUGININSTANCEOWNER_IID)

  /* void setInstance (in nsNPAPIPluginInstancePtr aInstance); */
  NS_IMETHOD SetInstance(nsNPAPIPluginInstance * aInstance) = 0;

  /* [nostdcall,notxpcom] nsNPAPIPluginInstancePtr getInstance (); */
  virtual nsNPAPIPluginInstance * GetInstance(void) = 0;

  /* void getWindow (in NPWindowStarRef aWindow); */
  NS_IMETHOD GetWindow(NPWindow * & aWindow) = 0;

  /* readonly attribute int32_t mode; */
  NS_IMETHOD GetMode(int32_t *aMode) = 0;

  /* void createWidget (); */
  NS_IMETHOD CreateWidget(void) = 0;

   /**
   * Called when there is a valid target so that the proper
   * frame can be updated with new content. will not be called
   * with nullptr aTarget.
   */
  NS_IMETHOD
  GetURL(const char *aURL, const char *aTarget,
         nsIInputStream *aPostStream,
         void *aHeadersData, uint32_t aHeadersDataLen,
         bool aDoCheckLoadURIChecks) = 0;
  /* readonly attribute Document document; */
  NS_IMETHOD GetDocument(mozilla::dom::Document **aDocument) = 0;

  /* void invalidateRect (in NPRectPtr aRect); */
  NS_IMETHOD InvalidateRect(NPRect * aRect) = 0;

  /* void invalidateRegion (in NPRegion aRegion); */
  NS_IMETHOD InvalidateRegion(NPRegion aRegion) = 0;

  /* void redrawPlugin (); */
  NS_IMETHOD RedrawPlugin(void) = 0;

  /* void getNetscapeWindow (in voidPtr aValue); */
  NS_IMETHOD GetNetscapeWindow(void * aValue) = 0;

   virtual NPBool  ConvertPoint(double sourceX, double sourceY, NPCoordinateSpace sourceSpace,
                               double *destX, double *destY, NPCoordinateSpace destSpace) = 0;
  virtual NPError InitAsyncSurface(NPSize *size, NPImageFormat format,
                                   void *initData, NPAsyncSurface *surface) = 0;
  virtual NPError FinalizeAsyncSurface(NPAsyncSurface *surface) = 0;
  virtual void SetCurrentAsyncSurface(NPAsyncSurface *surface, NPRect *changed) = 0;
  /* void setEventModel (in int32_t eventModel); */
  NS_IMETHOD SetEventModel(int32_t eventModel) = 0;

  /* void callSetWindow (); */
  NS_IMETHOD CallSetWindow(void) = 0;

  /* double getContentsScaleFactor (); */
  NS_IMETHOD GetContentsScaleFactor(double *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPluginInstanceOwner, NS_IPLUGININSTANCEOWNER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPLUGININSTANCEOWNER \
  NS_IMETHOD SetInstance(nsNPAPIPluginInstance * aInstance) override; \
  virtual nsNPAPIPluginInstance * GetInstance(void) override; \
  NS_IMETHOD GetWindow(NPWindow * & aWindow) override; \
  NS_IMETHOD GetMode(int32_t *aMode) override; \
  NS_IMETHOD CreateWidget(void) override; \
  NS_IMETHOD GetDocument(mozilla::dom::Document **aDocument) override; \
  NS_IMETHOD InvalidateRect(NPRect * aRect) override; \
  NS_IMETHOD InvalidateRegion(NPRegion aRegion) override; \
  NS_IMETHOD RedrawPlugin(void) override; \
  NS_IMETHOD GetNetscapeWindow(void * aValue) override; \
  NS_IMETHOD SetEventModel(int32_t eventModel) override; \
  NS_IMETHOD CallSetWindow(void) override; \
  NS_IMETHOD GetContentsScaleFactor(double *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPLUGININSTANCEOWNER \
  nsresult SetInstance(nsNPAPIPluginInstance * aInstance); \
  nsNPAPIPluginInstance * GetInstance(void); \
  nsresult GetWindow(NPWindow * & aWindow); \
  nsresult GetMode(int32_t *aMode); \
  nsresult CreateWidget(void); \
  nsresult GetDocument(mozilla::dom::Document **aDocument); \
  nsresult InvalidateRect(NPRect * aRect); \
  nsresult InvalidateRegion(NPRegion aRegion); \
  nsresult RedrawPlugin(void); \
  nsresult GetNetscapeWindow(void * aValue); \
  nsresult SetEventModel(int32_t eventModel); \
  nsresult CallSetWindow(void); \
  nsresult GetContentsScaleFactor(double *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPLUGININSTANCEOWNER(_to) \
  NS_IMETHOD SetInstance(nsNPAPIPluginInstance * aInstance) override { return _to SetInstance(aInstance); } \
  virtual nsNPAPIPluginInstance * GetInstance(void) override { return _to GetInstance(); } \
  NS_IMETHOD GetWindow(NPWindow * & aWindow) override { return _to GetWindow(aWindow); } \
  NS_IMETHOD GetMode(int32_t *aMode) override { return _to GetMode(aMode); } \
  NS_IMETHOD CreateWidget(void) override { return _to CreateWidget(); } \
  NS_IMETHOD GetDocument(mozilla::dom::Document **aDocument) override { return _to GetDocument(aDocument); } \
  NS_IMETHOD InvalidateRect(NPRect * aRect) override { return _to InvalidateRect(aRect); } \
  NS_IMETHOD InvalidateRegion(NPRegion aRegion) override { return _to InvalidateRegion(aRegion); } \
  NS_IMETHOD RedrawPlugin(void) override { return _to RedrawPlugin(); } \
  NS_IMETHOD GetNetscapeWindow(void * aValue) override { return _to GetNetscapeWindow(aValue); } \
  NS_IMETHOD SetEventModel(int32_t eventModel) override { return _to SetEventModel(eventModel); } \
  NS_IMETHOD CallSetWindow(void) override { return _to CallSetWindow(); } \
  NS_IMETHOD GetContentsScaleFactor(double *_retval) override { return _to GetContentsScaleFactor(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPLUGININSTANCEOWNER(_to) \
  NS_IMETHOD SetInstance(nsNPAPIPluginInstance * aInstance) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetInstance(aInstance); } \
  virtual nsNPAPIPluginInstance * GetInstance(void) override; \
  NS_IMETHOD GetWindow(NPWindow * & aWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWindow(aWindow); } \
  NS_IMETHOD GetMode(int32_t *aMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMode(aMode); } \
  NS_IMETHOD CreateWidget(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateWidget(); } \
  NS_IMETHOD GetDocument(mozilla::dom::Document **aDocument) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDocument(aDocument); } \
  NS_IMETHOD InvalidateRect(NPRect * aRect) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InvalidateRect(aRect); } \
  NS_IMETHOD InvalidateRegion(NPRegion aRegion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InvalidateRegion(aRegion); } \
  NS_IMETHOD RedrawPlugin(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RedrawPlugin(); } \
  NS_IMETHOD GetNetscapeWindow(void * aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNetscapeWindow(aValue); } \
  NS_IMETHOD SetEventModel(int32_t eventModel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEventModel(eventModel); } \
  NS_IMETHOD CallSetWindow(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CallSetWindow(); } \
  NS_IMETHOD GetContentsScaleFactor(double *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentsScaleFactor(_retval); } 


#endif /* __gen_nsIPluginInstanceOwner_h__ */
