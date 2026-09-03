# Resid

<div dir="rtl">

html از فایل pdf برای تولید فایل cli یک ابزار**Resid**
</div>

## نحوه استفاده

```bash
resid --create <output.pdf> --from <input.html>
```

<div dir="rtl">

برای مثال:

</div>

```bash
resid --create invoice.pdf --from invoice.html
```

<div dir="rtl">

## مثال

فایل `invoice.html`:

</div>

```html
<!DOCTYPE html>
<html lang="fa" dir="rtl">

<head>
  <style>
    body {
      direction: rtl;
      font-family: "B-Nazanin";
      font-size: 10pt;
      line-height: 1.2;
    }

    .invoice {
      margin: 1pt;
      border: 1pt solid #546E7A;
    }

    .header {
      margin: 1pt;
      padding: 1pt;
      text-align: center;
      background: #B0BEC5;
      border: 1pt solid #546E7A;
    }

    .title {
      margin: 1pt;
      padding: 1pt;
      font-size: 22pt;
      text-align: center;
    }

    .subtitle {
      margin: 1pt;
      font-size: 11pt;
      text-align: center;
    }

    .section {
      margin: 1pt;
      padding: 1pt;
      background: #EEEEEE;
      border: 1pt solid #546E7A;
    }

    .section-title {
      margin: 1pt;
      padding: 1pt;
      font-size: 15pt;
      text-align: right;
    }

    .text {
      margin: 1pt;
      padding: 10pt;
      font-size: 12pt;
      line-height: 1.7;
      text-align: right;
    }

    .total {
      margin: 1pt;
      padding: 1pt;
      background: #4DB6AC;
      border: 1pt solid #C8E6C9;
      font-size: 15pt;
      text-align: center;
    }

    .footer {
      text-align: center;
      padding: 1pt;
      font-size: 10pt;
    }
  </style>
</head>

<body>
  <div class="invoice">

    <div class="header">
      <div class="title">فاکتور فروش</div>
      <div class="subtitle">شماره فاکتور: ۱۴۰۵-۰۰۱۲۵</div>
      <div class="subtitle">تاریخ: ۱۴۰۵/۰۶/۱۲</div>
    </div>

    <div class="section">
      <div class="section-title">مشخصات فروشنده</div>
      <div class="text">
        شرکت نرم‌افزاری نمونه، ارائه‌دهنده خدمات طراحی و توسعه نرم‌افزار
      </div>
      <div class="text">شماره تماس: ۰۲۱-۱۲۳۴۵۶۷۸</div>
      <div class="text">آدرس: تهران، خیابان نمونه، ساختمان شماره ۱۰</div>
    </div>

    <div class="section">
      <div class="section-title">مشخصات مشتری</div>
      <div class="text">نام مشتری: علی رضایی</div>
      <div class="text">شماره تماس: ۰۹۱۲۱۲۳۴۵۶۷</div>
      <div class="text">آدرس: تهران، خیابان آزادی، کوچه دهم</div>
    </div>

    <div class="section">
      <div class="section-title">شرح خدمات</div>

      <div class="text">
        طراحی و پیاده‌سازی سامانه مدیریت سفارش‌ها، توسعه رابط برنامه‌نویسی کاربردی، ایجاد
        سیستم احراز هویت و تهیه گزارش‌های مدیریتی. این متن عمداً طولانی است تا قابلیت شکست
        خطوط، اندازه‌گیری صحیح متن فارسی، فاصله‌گذاری عمودی و قرارگیری راست‌به‌چپ در چند خط
        مختلف صفحه آزمایش شود.
      </div>

      <div class="text">
        خدمات شامل تحلیل نیازمندی‌ها، طراحی معماری نرم‌افزار، پیاده‌سازی سرویس‌های اصلی،
        آزمایش عملکرد و آماده‌سازی نسخه نهایی برای استفاده در محیط عملیاتی است.
      </div>
    </div>

    <div class="section">
      <div class="section-title">شرایط پرداخت</div>

      <div class="text">
        مبلغ کل فاکتور پس از تأیید نهایی مشتری قابل پرداخت است و کلیه خدمات درج‌شده در این
        فاکتور مطابق توافق طرفین ارائه خواهد شد.
      </div>
    </div>

    <div class="total">
      مبلغ قابل پرداخت: ۱۲۵,۰۰۰,۰۰۰ ریال
    </div>

    <div class="footer">
      از خرید و اعتماد شما سپاسگزاریم.
    </div>

  </div>
</body>

</html>
```

<div dir="rtl">

برای تبدیل فایل:

</div>

```bash
resid --create invoice.pdf --from invoice.html
```

<div dir="rtl">

## HTML موارد پشتیبانی شده

در حال حاضر فیچر های زیر را پشتیبانی می شود.

</div>

```text
html
body
header
footer
main
section
article
aside
nav
div
p
h1
h2
h3
h4
h5
h6

table
thead
tbody
tfoot
tr
td
th

id
class
style
dir
```

## CSS موارد پشتیبانی شده



</div>

```text
display
direction

font-family
font-size
font-weight
line-height
text-align

color
background
background-color
margin
padding

border
```

## فونت
در این نسخه فقط از فونت  بی نازنین پشتیبانی می شود:
</div>

```css
body {
    font-family: "B-Nazanin";
}
```
