<h1 dir="rtl" align="right">Network Manager</h1>

<div dir="ltr" align="left">
  <img src="https://github.com/ussefT/NetworkManager/blob/main/image.png" />
</div>

<p dir="rtl" align="right">یک برنامهٔ دسکتاپ کوچک برای لینوکس که کمک می‌کند فعالیت شبکه را زیر نظر داشته باشید. نتورک منیجر با <code dir="ltr">Rust</code> و <code dir="ltr">GTK4</code> ساخته شده و هم یک داشبورد کامل دارد و هم یک پنجرهٔ جمع‌وجور که از نوار اعلان‌ها باز می‌شود.</p>

<p dir="rtl" align="right"><a href="https://github.com/ussefT/NetworkManager/README.md">En</a></p>

<blockquote dir="rtl" align="right"><strong>وضعیت:</strong> در ابتدای توسعه (<code dir="ltr">0.1.0</code>) است؛ ممکن است ظاهر و رفتار برنامه در نسخه‌های بعدی تغییر کند.</blockquote>

<h2 dir="rtl" align="right">امکانات</h2>

<ul dir="rtl" align="right">
  <li>رابط‌های شبکه‌ای را که هنگام اجرای برنامه شناسایی شده‌اند فهرست می‌کند و امکان انتخاب یکی از آن‌ها را می‌دهد.</li>
  <li>مقادیر دریافت و ارسال داده، مجموع انتقال داده و نمودار فعالیت شبکه را نشان می‌دهد.</li>
  <li>داشبورد کامل و پنجره‌ای کوچک‌تر دارد. رابط انتخاب‌شده و وضعیت پایش در هر دو پنجره هماهنگ است.</li>
  <li>امکان توقف موقت و ازسرگیری پایش را فراهم می‌کند.</li>
  <li>از پوسته‌های روشن و تاریک پشتیبانی می‌کند.</li>
  <li>یک آیکون در نوار اعلان‌ها ایجاد می‌کند. با کلیک روی آن می‌توانید پنجرهٔ کوچک را نمایش دهید یا پنهان کنید؛ از منوی آیکون هم می‌توانید داشبورد کامل را باز کنید یا از برنامه خارج شوید.</li>
  <li>با بستن پنجره، داشبورد پنهان می‌شود تا برنامه در نوار اعلان‌ها در دسترس بماند.</li>
</ul>

<h2 dir="rtl" align="right">نکته‌ای دربارهٔ مقادیر نمایش‌داده‌شده</h2>

<p dir="rtl" align="right">در نسخهٔ فعلی، مقادیر دریافت و ارسال با عنوان سرعت نمایش داده می‌شوند، اما از شمارندهٔ تجمعی بایت‌های دریافتی و ارسالی سیستم به دست می‌آیند؛ بنابراین هنوز سرعت واقعی در هر ثانیه نیستند. نمودار هم همین مقادیر را رسم می‌کند. بخش مجموع دریافت و ارسال نیز شمارنده‌های تجمعی انتقال داده را نشان می‌دهد.</p>

<h2 dir="rtl" align="right">پیش‌نیازها</h2>

<ul dir="rtl" align="right">
  <li>یک محیط دسکتاپ لینوکس با <code dir="ltr">GTK4</code> و پشتیبانی از نوار اعلان‌ها یا <code dir="ltr">Status Notifier</code>.</li>
  <li><code dir="ltr">Rust</code> و <code dir="ltr">Cargo</code> (نسخهٔ <code dir="ltr">Rust 1.85</code> یا جدیدتر برای پشتیبانی از <code dir="ltr">Rust 2024</code>).</li>
  <li>فایل‌های توسعهٔ <code dir="ltr">GTK4</code> و <code dir="ltr">pkg-config</code> که می‌توانید با مدیر بستهٔ توزیع لینوکس خود نصب کنید.</li>
</ul>

<p dir="rtl" align="right">برای نصب پیش‌نیازهای ساخت GTK در Debian یا Ubuntu:</p>

```bash
sudo apt install build-essential pkg-config libgtk-4-dev
```

<p dir="rtl" align="right">اگر Rust روی سیستم شما نصب نیست، آن را از طریق <a href="https://rustup.rs/">rustup</a> نصب کنید.</p>

<h2 dir="rtl" align="right">مستندات کتابخانه‌ها</h2>

<ul dir="rtl" align="right">
  <li><a href="https://docs.rs/gtk4/0.11.5/gtk4/"><code dir="ltr">gtk4</code></a> — اتصال‌های GTK4 برای Rust؛ برای ساخت رابط دسکتاپ استفاده شده است.</li>
  <li><a href="https://docs.rs/sysinfo/0.39.6/sysinfo/"><code dir="ltr">sysinfo</code></a> — کتابخانهٔ اطلاعات سیستم؛ برای خواندن شمارنده‌های رابط شبکه استفاده شده است.</li>
  <li><a href="https://docs.rs/tray-icon/0.26.1/tray_icon/"><code dir="ltr">tray-icon</code></a> — برای پشتیبانی از آیکون و منوی نوار اعلان‌ها.</li>
</ul>

<h2 dir="rtl" align="right">ساخت و اجرا</h2>

<p dir="rtl" align="right">در پوشهٔ پروژه دستورهای زیر را اجرا کنید:</p>

```bash
cargo check
cargo run
```

<p dir="rtl" align="right">برای ساخت نسخهٔ بهینه‌شده:</p>

```bash
cargo build --release
```

<p dir="rtl" align="right">فایل اجرایی ساخته‌شده در مسیر <code dir="ltr">target/release/my-gtk-app</code> قرار می‌گیرد.</p>

<h2 dir="rtl" align="right">ساختار پروژه</h2>

<ul dir="rtl" align="right">
  <li><code dir="ltr">src/main.rs</code> آیکون نوار اعلان‌ها و برنامهٔ GTK را راه‌اندازی می‌کند.</li>
  <li><code dir="ltr">src/ui.rs</code> داشبوردها، نمودار، کنترل‌ها و پوسته‌ها را می‌سازد.</li>
  <li><code dir="ltr">src/network.rs</code> شمارنده‌های رابط‌های شبکه را با <code dir="ltr">sysinfo</code> می‌خواند.</li>
  <li><code dir="ltr">src/tray.rs</code> آیکون نوار اعلان‌ها و منوی آن را ایجاد می‌کند.</li>
</ul>

<h2 dir="rtl" align="right">مشارکت در پروژه</h2>

<p dir="rtl" align="right">از مشارکت شما خیلی خوشحال می‌شویم! اگر باگی پیدا کرده‌اید، ایدهٔ مفیدی دارید، می‌خواهید مستندات را بهتر کنید یا به هر شکل دیگری دوست دارید به بهبود نتورک منیجر کمک کنید، پیشنهادتان برای ما ارزشمند است.</p>

<h3 dir="rtl" align="right">باگ پیدا کرده‌اید یا پیشنهادی دارید؟</h3>

<p dir="rtl" align="right">لطفاً <a href="https://github.com/ussefT/NetworkManager/issues">یک Issue ثبت کنید</a>. اگر این موارد را هم بنویسید، بررسی و پیگیری موضوع آسان‌تر می‌شود:</p>

<ul dir="rtl" align="right">
  <li>انتظار داشتید چه اتفاقی بیفتد و در عمل چه اتفاقی افتاد؟</li>
  <li>برای بازتولید مشکل چه مراحلی را باید انجام داد؟</li>
  <li>از چه توزیع لینوکس و محیط دسکتاپی استفاده می‌کنید؟</li>
  <li>پیام خطا یا تصویر مرتبطی دارید؟</li>
</ul>

<p dir="rtl" align="right">برای پیشنهاد قابلیت تازه، توضیح دهید این تغییر قرار است چه مشکلی را حل کند. اگر هنوز همهٔ جزئیات را نمی‌دانید اشکالی ندارد؛ می‌توانید Issue را فقط برای شروع گفت‌وگو باز کنید.</p>

<h3 dir="rtl" align="right">دوست دارید در کدنویسی مشارکت کنید؟</h3>

<ol dir="rtl" align="right">
  <li>برای تغییرهای بزرگ‌تر، ابتدا یک Issue باز کنید تا دربارهٔ راهکار پیشنهادی گفت‌وگو کنیم.</li>
  <li>مخزن را Fork کنید و برای تغییرتان یک شاخه بسازید.</li>
  <li>تغییرتان را انجام دهید و دستور <code dir="ltr">cargo check</code> را اجرا کنید.</li>
  <li>یک Pull Request باز کنید و کوتاه توضیح دهید چه چیزی را تغییر داده‌اید و دلیلش چه بوده است.</li>
</ol>

<p dir="rtl" align="right">اصلاح‌های کوچک و بهبود مستندات هم به گرمی پذیرفته می‌شوند. از وقتی که برای مشارکت می‌گذارید، سپاسگزاریم!</p>