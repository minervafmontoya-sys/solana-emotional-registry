Emotional Registry: Mi Primer CRUD en Solana
¡Hola! Este es el backend de mi proyecto para la certificación. Es un programa descentralizado (Smart Contract) hecho con Rust y Anchor. La idea es sencilla: cualquier persona con una wallet puede guardar su "estado emocional" o una opinión en la blockchain de forma permanente (o borrarla si cambia de opinión).
Me enfoqué en que sea eficiente con el almacenamiento y, sobre todo, seguro.
¿Qué incluye este proyecto?
Para cumplir con la certificación, me aseguré de cubrir estos puntos:
•	Código 100% Rust: Todo el programa corre on-chain.
•	CRUD Completo: Puedes crear, leer, actualizar y borrar registros.
•	Uso de PDAs: Las cuentas no son aleatorias; se generan de forma inteligente para que cada usuario tenga la suya propia.
•	Gestión de Renta: Si borras tu registro, el programa te devuelve los SOL que se usaron para apartar el espacio.
Detalles Técnicos (Para los curiosos)
1. ¿Cómo guardamos los datos?
Usamos una estructura llamada EmotionEntry que guarda:
•	author: Quién creó el registro (tu dirección de wallet).
•	emotion: Un texto corto (máximo 50 caracteres) con tu emoción.
•	description: Espacio para explayarse un poco más (hasta 200 caracteres).
•	timestamp: El momento exacto en el que se hizo el registro.
2. El truco de las PDAs
Para que no haya líos de "quién es qué cuenta", las direcciones se crean usando:
1.	La palabra mágica "emotion".
2.	Tu Public Key.
Esto significa que tu registro es único y nadie más puede editarlo porque el programa verifica que tú seas el dueño antes de cualquier cambio.
¿Cómo hacerlo funcionar?
Si quieres probarlo, sigue estos pasos:
1.	Prepárate: Ten instalados Rust y Anchor.
2.	Construye: Tira un anchor build en la terminal.
3.	Despliega: Súbelo a la Devnet con anchor deploy.
4.	Prueba: Corre los tests en TypeScript para ver cómo ocurre la magia en tiempo real.

# solana-emotional-registry
