import 'package:dart_frog/dart_frog.dart';

import '../../../lib/user.dart';

Response onRequest(RequestContext context) {
    final user = context.read<User>();
    return Response(body: user);
}
