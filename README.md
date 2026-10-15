# Resid

Resid یک ابزار خط فرمان برای تولید PDF از HTML و CSS است. این پروژه برای اسناد فارسی و راست‌به‌چپ مانند فاکتور، رسید و گزارش مناسب است.

## ساخت

```bash
cargo build --release
```

فایل اجرایی:

```text
target/release/resid
```

## استفاده

```bash
resid --create output.pdf --from input.html
```

یا:

```bash
resid -c output.pdf -f input.html
```

## گزینه‌ها

```text
--from, -f     مسیر فایل HTML یا رشتهٔ HTML
--create, -c   مسیر فایل PDF خروجی
--font         نام فونت (پیش‌فرض: Vazirmatn)
--page         اندازهٔ صفحه (پیش‌فرض: a4)
```

اندازه‌های صفحه:

```text
a3, a4, a4-landscape, a5, a6
```

فونت‌های پشتیبانی‌شده:

```text
Estedad, Gandom, Parastoo, Samim, Shabnam, Vazirmatn
```

## مثال کوچک

```html
<!DOCTYPE html>
<html lang="fa" dir="rtl">
<head>
  <style>
    body {
      direction: rtl;
      font-family: "Vazirmatn";
      font-size: 12pt;
    }
    h1 { color: #0f4c81; }
  </style>
</head>
<body>
  <h1>فاکتور نمونه</h1>
  <p>شماره: 021-4844848</p>
</body>
</html>
```

```bash
resid --from invoice.html --create invoice.pdf --font Vazirmatn
```

