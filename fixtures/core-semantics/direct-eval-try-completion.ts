function run() {
  let marker = 0;
  let normal = eval("try { marker = 1; marker; } catch (e) { 99; } finally { marker = marker + 1; marker; }");
  console.log(normal);
  console.log(marker);

  let caught = eval("try { throw 5; } catch (e) { marker = e + 2; marker; } finally { marker = marker + 1; marker; }");
  console.log(caught);
  console.log(marker);
}

run();
