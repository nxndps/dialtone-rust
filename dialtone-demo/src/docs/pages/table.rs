use super::*;

const TABLE_BODY: &str = "<thead>\n    <tr>\n        <th>\"Name\"</th>\n        <th>\"Role\"</th>\n    </tr>\n</thead>\n<tbody>\n    <tr>\n        <td>\"Ada Lovelace\"</td>\n        <td>\"Primary\"</td>\n    </tr>\n    <tr>\n        <td>\"Grace Hopper\"</td>\n        <td>\"Secondary\"</td>\n    </tr>\n    <tr>\n        <td>\"Alan Turing\"</td>\n        <td>\"Escalation\"</td>\n    </tr>\n</tbody>";

fn table_rows() -> impl IntoView {
    view! {
        <thead>
            <tr>
                <th>"Name"</th>
                <th>"Role"</th>
            </tr>
        </thead>
        <tbody>
            <tr>
                <td>"Ada Lovelace"</td>
                <td>"Primary"</td>
            </tr>
            <tr>
                <td>"Grace Hopper"</td>
                <td>"Secondary"</td>
            </tr>
            <tr>
                <td>"Alan Turing"</td>
                <td>"Escalation"</td>
            </tr>
        </tbody>
    }
}

#[component]
pub(crate) fn TablePage() -> impl IntoView {
    provide_context(VariantSource(include_str!("table.rs")));
    let caption = RwSignal::new("On call".to_string());
    let striped = RwSignal::new(true);
    let root_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                view! {
                    <DtTable
                        caption=caption.get()
                        striped=striped.get()
                        class=root_class.get()
                    >
                        {table_rows()}
                    </DtTable>
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Caption" hint="Empty omits the caption." value=caption />
            <SettingCheck label="Striped" value=striped />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "caption", &caption.get());
        push_flag(&mut props, "striped", striped.get());
        push_str(&mut props, "class", &root_class.get());
        element("DtTable", &props, Some(TABLE_BODY))
    });
    let classes = class_panel(vec![ClassField {
        label: "Class",
        hint: ROOT_CLASS,
        value: root_class,
        off: None,
    }]);
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">
                "There is no Vue DtTable. This is the d-table CSS component. striped adds d-table--striped. d-table--inverted is deprecated. Wrap the table in DtModeIsland to invert it."
            </p>
            <Play preview=preview settings=settings classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Plain">
                    <DtTable caption="Team">{table_rows()}</DtTable>
                </Variant>
                <Variant title="Striped">
                    <DtTable caption="Team" striped=true>{table_rows()}</DtTable>
                </Variant>
                <Variant title="Inverted">
                    <DtModeIsland>
                        <DtTable caption="Team" striped=true>{table_rows()}</DtTable>
                    </DtModeIsland>
                </Variant>
            </div>
        </section>
    }
}
