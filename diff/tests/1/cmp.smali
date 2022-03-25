.class Lbttv/SleepTimer$2;
.super Ljava/lang/Object;
.source "SleepTimer.java"

# interfaces
.implements Landroid/content/DialogInterface$OnClickListener;


# annotations
.annotation system Ldalvik/annotation/EnclosingMethod;
    value = Lbttv/SleepTimer;->openSelectDialog(Landroid/content/Context;)V
.end annotation

.annotation system Ldalvik/annotation/InnerClass;
    accessFlags = 0x0
    name = null
.end annotation


# instance fields
.field final synthetic val$minutes:[I

.field final synthetic val$selected:[I


# direct methods
.method constructor <init>([I[I)V
    .registers 3

    .line 73
    iput-object p1, p0, Lbttv/SleepTimer$2;->val$selected:[I

    iput-object p2, p0, Lbttv/SleepTimer$2;->val$minutes:[I

    invoke-direct {p0}, Ljava/lang/Object;-><init>()V

    return-void
.end method
