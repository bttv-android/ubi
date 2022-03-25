.class abstract Lbttv/SleepTimer$2;
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
.method constructor <init>(I)V
    .registers 3

    .line 73
    iput-object p1, p0, Lbttv/SleepTimer$2;->val$selected:[I

    iput-object p2, p0, Lbttv/SleepTimer$2;->val$minutes:[I

    invoke-direct {p0}, Ljava/lang/Object;-><init>()V

    return-void
.end method


# virtual methods
.method public onClick(Landroid/content/DialogInterface;I)V
    .registers 5

    .line 75
    iget-object p1, p0, Lbttv/SleepTimer$2;->val$selected:[I

    const/4 v0, 0x0

    aput p2, p1, v0

    .line 76
    aget p2, p1, v0

    const/4 v1, -0x1

    if-eq p2, v1, :cond_13

    .line 77
    iget-object p2, p0, Lbttv/SleepTimer$2;->val$minutes:[I

    aget p1, p1, v0

    aget p1, p2, p1

    # invokes: Lbttv/SleepTimer;->scheduleStop(I)V
    invoke-static {p1}, Lbttv/SleepTimer;->access$100(I)V

    :cond_13
    return-void
.end method
