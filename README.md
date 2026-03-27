# 🚀 Welcome to TimeLocky! 🚀

Hey there! So you're ready to dive into the world of Solana token time-locking and vesting? Awesome! TimeLocky is here to make that journey as smooth as possible. This guide will get you from zero to hero in no time.

## ✅ What You'll Need

Before we get our hands dirty, let's make sure you have a couple of things installed and ready to go:

*   **🐍 Python (version 3.10 or higher):** This is the engine that powers our app. If you're not sure what version you have, just open a terminal and type `python --version`.
*   **🦀 Rust:** This gives our app its lightning-fast core. You can check your version with `rustc --version`. Don't have it? No sweat! The app has a pure Python fallback, but it's definitely zippier with Rust.

## 🛠️ Let's Get Building!

Alright, time for the fun part. Follow these steps, and you'll be up and running in a flash.

1.  **Clone the Magic:**
    First, you'll need to get the code onto your machine.
    ```bash
    git clone <your-repository-url>
    cd <your-repository-name>
    ```

2.  **Set Up Your Space:**
    It's always a good idea to create a virtual environment to keep things tidy.
    ```bash
    # Create the environment
    python -m venv venv
    
    # Activate it (the command is a bit different for Windows)
    # On macOS/Linux:
    source venv/bin/activate
    # On Windows:
    venv\Scripts\activate
    ```
    Now, let's install all the Python goodies the app needs:
    ```bash
    pip install -r requirements.txt
    ```

3.  **Power Up the Rust Core:**
    Our app is smart and will try to build the Rust part for you automatically. But if you're feeling hands-on, you can do it yourself.
    ```bash
    maturin develop
    ```

## 🖥️ Time to Launch!

With the setup out of the way, you're ready to start the app. Just run this command:

```bash
python main.py
```

You should see the TimeLocky window pop up. Here's a quick tour of what you can do:

*   **👛 Wallet Central:** Hook up your Solana wallet and manage your digital treasures.
*   **⏳ Create Time Locks:** Lock up your SPL tokens and set them to release whenever you want.
*   **📈 Manage Streams:** Keep an eye on all your active time locks and vesting schedules.
*   **📊 Stat-o-Matic:** Get cool charts and numbers about your locked assets.
*   **📓 Address Book:** Your personal rolodex for all your Solana contacts.

---

And that's it! We're so excited for you to start using TimeLocky. If you run into any bumps along the way, don't hesitate to check out the project's issue tracker. Happy locking!

