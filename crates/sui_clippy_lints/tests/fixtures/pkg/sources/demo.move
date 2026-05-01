module lint_fixture::demo {
    // TODO: implement

    #[test_only]
    const T: u64 = 0;

    public fun send_to_self(ctx: &sui::tx_context::TxContext) {
        let _ = sui::tx_context::sender(ctx);
        transfer::public_transfer(0, sui::tx_context::sender(ctx));
        abort 0;
    }

    public fun touch_dynamic_field() {
        let _ = sui::dynamic_field::borrow_mut;
    }

    public fun use_clock() {
        let _ = sui::clock::timestamp_ms;
    }
}
