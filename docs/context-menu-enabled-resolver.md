# ContextMenuの有効状態を表示直前に解決する

公開host projection leaseの`with_context_menu_enabled_resolver`へ、項目IDと静的な有効状態を受けて最新の有効状態を返す関数を渡す。
メニューの配置、開閉、描画、focus、入力処理、AccessKitはKUCが所有する。

```rust
use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurfaceHostProjectionLease,
    EguiTextCommandSurfacePresentationToken,
};
use std::{cell::Cell, rc::Rc};

fn lease(
    token: EguiTextCommandSurfacePresentationToken,
    action_enabled: Rc<Cell<bool>>,
) -> EguiTextCommandSurfaceHostProjectionLease {
    EguiTextCommandSurfaceHostProjectionLease::new(token, |_context| Ok(None))
        .with_context_menu_enabled_resolver(move |item_id, static_enabled| {
            Ok(if item_id == "action" {
                action_enabled.get()
            } else {
                static_enabled
            })
        })
}
```

resolverを指定しないleaseは、既存の静的presentationをそのまま利用する。
resolverはframe開始時に閉じたメニューを照会せず、開いているか、そのframeで新しく開く場合に最新値を解決する。
Escapeまたはメニュー外クリックで閉じるframeは、KUCが閉鎖操作を先に処理し、resolverを照会しない。resolverが直前に失敗していても閉じることができ、閉鎖完了後のframeでも照会しない。
同じ解決結果を、そのframeの表示・操作可否・AccessKit・記録へ適用する。
項目のID、表示名、構造や配置をresolverから変更することはできない。

resolverの失敗は`KucContextMenuEnabledResolverError`として検出でき、公開rootの表示は専用の型付きエラーを返す。
失敗したframeで古い有効状態を成功結果として返さない。次の表示では最新状態を再度解決する。
新しいleaseはresolverも置き換え、resolverなしのleaseは静的投影へ戻す。
拒否されたstale・duplicate・異rootのleaseは、現在のresolverを置き換えない。

この関数は有効状態の照会用とし、操作自体のhost効果は既存の一度限りのイベント転送経路で処理する。
