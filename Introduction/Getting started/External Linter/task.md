The IntelliJ Rust plugin doesn't detect all errors on its own; it relies on the Rust compiler for that. While learning Rust, it is helpful to see errors as you type. To achieve this, we recommend enabling an external linter as follows.

Once you do that, %IDE_NAME% will report all errors detected by either the IntelliJ Rust plugin or the Rust compiler.

The steps may vary slightly depending on the IDE you're using.

### RustRover
1. Go to **Settings / Preferences | Rust | External Linters**.
2. Set the parameters as follows:
   - Check the **Run external linter on the fly** box.
   - In the **External Tool** list, select **Cargo Check**.
![External Linters](images/rustrover_external_linters.png)
3. Press **OK**.

### IntelliJ IDEA
1. Go to **Settings / Preferences | Languages & Frameworks | Rust | External Linters**.
2. Set the parameters as follows:
   - Check the **Run external linter on the fly** box.
   - In the **External Tool** list, select **Cargo Check**.
![External Linters](images/idea_external_linters.png)
3. Press **OK**.
